use std::{
    cmp::Ordering,
    io::{Error, ErrorKind},
};

use database::{
    managers::spot::assets::AssetsManager,
    projections::spot::asset::Asset,
    sqlx::{Pool, Postgres},
    traits::TableManager,
};
use futures::TryStreamExt;
use ordered_float::OrderedFloat;
use regex::Regex;
use strsim::jaro_winkler;

use crate::api::public::search::AssetResponse;

#[derive(Clone)]
pub struct AssetPairRecognition {
    regex_fillter: Regex,
    assets_manager: AssetsManager,
}

impl AssetPairRecognition {
    pub fn new(database: Pool<Postgres>, regex_fillter: Regex) -> Self {
        Self {
            regex_fillter,
            assets_manager: AssetsManager::new(database),
        }
    }

    async fn get_assets(&self) -> Result<Vec<Asset>, Error> {
        self.assets_manager
            .get_all()
            .try_collect()
            .await
            .map_err(|err| Error::new(ErrorKind::BrokenPipe, err))
    }

    fn recognize_first<'a>(
        &self,
        phrase: &str,
        assets: &'a [Asset],
    ) -> Vec<(OrderedFloat<f64>, (&'a Asset, String))> {
        let iter_phrase = assets.iter().map(|asset| {
            (
                asset,
                (asset.symbol.to_lowercase(), asset.name.to_lowercase()),
            )
        });

        iter_phrase
            .map(|pair| {
                let symbol_sim = OrderedFloat(jaro_winkler(phrase, &pair.1 .0));
                let name_sim = OrderedFloat(jaro_winkler(phrase, &pair.1 .1));
                match symbol_sim.cmp(&name_sim) {
                    Ordering::Greater => (symbol_sim, (pair.0, pair.1 .0)),
                    Ordering::Less => (name_sim, (pair.0, pair.1 .1)),
                    Ordering::Equal => (OrderedFloat(0_f64), (pair.0, "".to_string())),
                }
            })
            .collect()
    }

    pub async fn recognize(
        self,
        input: &str,
    ) -> Result<Vec<(OrderedFloat<f64>, (AssetResponse, AssetResponse))>, Error> {
        let assets = self.get_assets().await?;
        let phrase = self.regex_fillter.replace_all(input, "").to_lowercase();
        let mut result = Vec::<(OrderedFloat<f64>, (AssetResponse, AssetResponse))>::with_capacity(
            assets.len().pow(2),
        );

        for (score1, (asset1, phr1)) in self.recognize_first(phrase.as_str(), &assets).into_iter() {
            let rem_phr = phrase[std::cmp::min(phrase.len(), phr1.len())..].to_string();
            for (score2, (asset2, _phr2)) in
                self.recognize_first(rem_phr.as_str(), &assets).into_iter()
            {
                let score_sum = score1 * score2;
                if asset1 == asset2 || score_sum == OrderedFloat(0_f64) {
                    continue;
                }
                result.push((
                    score_sum,
                    (asset1.to_owned().into(), asset2.to_owned().into()),
                ));
            }
        }
        result.sort_by(|lhs, rhs| lhs.0.cmp(&rhs.0));
        Ok(result)
    }
}
