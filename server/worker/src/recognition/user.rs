use std::io::{Error, ErrorKind};

use database::{managers::users::UsersManager, projections::user::User};
use futures::TryStreamExt;
use ordered_float::OrderedFloat;
use regex::Regex;
use serde::Serialize;
use sqlx::{Pool, Postgres};
use strsim::jaro_winkler;

use crate::database;

#[derive(Debug, Clone)]
pub struct UserRecognition {
    regex_fillter: Regex,
    users_manager: UsersManager,
}

#[derive(Debug, Serialize)]
pub struct UserRecognitionResult {
    score: OrderedFloat<f64>,
    user: User,
}

impl UserRecognition {
    pub fn new(database: Pool<Postgres>, regex_fillter: Regex) -> Self {
        Self {
            regex_fillter,
            users_manager: UsersManager::new(database),
        }
    }

    async fn get_users(&self) -> Result<Vec<User>, Error> {
        self.users_manager
            .get_all()
            .try_collect()
            .await
            .map_err(|err| Error::new(ErrorKind::BrokenPipe, err))
    }

    fn recognize_first<'a>(
        &self,
        phrase: &str,
        users: &'a [User],
    ) -> Vec<(OrderedFloat<f64>, &'a User)> {
        let iter_phrase = users.iter().map(|user| {
            (
                user,
                vec![
                    Some(user.address.to_string().to_lowercase()),
                    user.name.as_ref().map(|f| f.to_lowercase()),
                    user.email.as_ref().map(|f| f.to_lowercase()),
                    user.phone.as_ref().map(|f| f.to_lowercase()),
                ],
            )
        });

        iter_phrase
            .map(|pair| {
                (
                    pair.1
                        .into_iter()
                        .map(|f| match f {
                            Some(f) => OrderedFloat(jaro_winkler(phrase, &f)),
                            None => OrderedFloat(0_f64),
                        })
                        .sum::<OrderedFloat<f64>>(),
                    pair.0,
                )
            })
            .collect()
    }

    pub async fn recognize(self, input: &str) -> Result<Vec<UserRecognitionResult>, Error> {
        let users = self.get_users().await?;
        let phrase = self.regex_fillter.replace_all(input, "").to_lowercase();
        let mut result = Vec::<UserRecognitionResult>::new();

        for (score, user) in self.recognize_first(phrase.as_str(), &users).into_iter() {
            result.push(UserRecognitionResult {
                score,
                user: user.clone(),
            });
        }

        result.sort_by(|lhs, rhs| rhs.score.cmp(&lhs.score));
        Ok(result)
    }
}
