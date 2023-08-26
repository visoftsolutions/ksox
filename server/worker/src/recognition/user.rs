use std::{io::{Error, ErrorKind}, future};

use database::{managers::users::UsersManager, projections::user::User};
use futures::{TryStreamExt};
use nalgebra::DVector;
use ordered_float::OrderedFloat;
use serde::Serialize;
use sqlx::{Pool, Postgres};
use strsim::sorensen_dice;
use uuid::Uuid;

use crate::database;

#[derive(Debug, Clone)]
pub struct UserRecognition {
    users_manager: UsersManager,
}

#[derive(Debug, Serialize)]
pub struct UserRecognitionResult {
    score: OrderedFloat<f64>,
    user: User,
}

impl UserRecognition {
    pub fn new(database: Pool<Postgres>) -> Self {
        Self {
            users_manager: UsersManager::new(database),
        }
    }

    async fn get_users(&self) -> Result<Vec<User>, Error> {
        self.users_manager
            .get_all()
            .try_filter(|user| future::ready( user.id != Uuid::default() ))
            .try_collect()
            .await
            .map_err(|err| Error::new(ErrorKind::BrokenPipe, err))
    }

    fn recognize_first<'a>(&self, phrase: &str, users: &'a [User]) -> Vec<(f64, &'a User)> {
        let iter_phrase = users.iter().map(|user| {
            (
                user,
                vec![
                    Some(user.id.to_string().to_lowercase()),
                    Some(user.address.to_string().to_lowercase()),
                    user.name.as_ref().map(|f| f.to_lowercase()),
                ],
            )
        });

        iter_phrase
            .map(|pair| {
                (
                    DVector::from_column_slice(
                        &pair
                            .1
                            .into_iter()
                            .map(|f| match f {
                                Some(f) => sorensen_dice(phrase, &f).abs(),
                                None => 0_f64,
                            })
                            .collect::<Vec<f64>>(),
                    )
                    .norm()
                    .abs(),
                    pair.0,
                )
            })
            .collect()
    }

    pub async fn recognize(self, input: &str) -> Result<Vec<UserRecognitionResult>, Error> {
        let users = self.get_users().await?;
        let mut result = Vec::<UserRecognitionResult>::new();

        for (score, user) in self.recognize_first(input, &users).into_iter() {
            result.push(UserRecognitionResult {
                score: OrderedFloat(score),
                user: user.clone(),
            });
        }

        result.sort_by(|lhs, rhs| rhs.score.cmp(&lhs.score));

        Ok(result)
    }
}
