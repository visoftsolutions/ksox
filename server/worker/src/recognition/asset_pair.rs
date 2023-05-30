use std::{
    cmp::Ordering,
    io::{Error, ErrorKind},
};

use database::{managers::assets::AssetsManager, projections::asset::Asset};
use futures::TryStreamExt;
use ordered_float::OrderedFloat;
use regex::Regex;
use serde::Serialize;
use sqlx::{Pool, Postgres};
use strsim::jaro_winkler;

use crate::{api::public::assets::search_asset_pair::AssetResponse, database};

#[derive(Debug, Clone)]
pub struct AssetPairRecognition {
    regex_fillter: Regex,
    assets_manager: AssetsManager,
}

#[derive(Debug, Serialize)]
pub struct AssetPairRecognitionResult {
    score: OrderedFloat<f64>,
    asset0: AssetResponse,
    asset1: AssetResponse,
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

    pub async fn recognize(self, input: &str) -> Result<Vec<AssetPairRecognitionResult>, Error> {
        let assets = self.get_assets().await?;
        let phrase = self.regex_fillter.replace_all(input, "").to_lowercase();
        let mut result = Vec::<AssetPairRecognitionResult>::new();

        for (score0, (asset0, phr1)) in self.recognize_first(phrase.as_str(), &assets).into_iter() {
            let rem_phr = phrase[std::cmp::min(phrase.len(), phr1.len())..].to_string();
            for (score1, (asset1, _phr2)) in self
                .recognize_first(rem_phr.as_str(), &assets)
                .into_iter()
                .map(|e| (e.0 * 0.9, e.1))
            {
                let score_sum = score0 + score1;
                if asset0 == asset1 {
                    continue;
                }
                result.push(AssetPairRecognitionResult {
                    score: score_sum,
                    asset0: asset0.to_owned().into(),
                    asset1: asset1.to_owned().into(),
                });
            }
        }
        result.sort_by(|lhs, rhs| rhs.score.cmp(&lhs.score));
        Ok(result)
    }
}
