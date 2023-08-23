use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
    Decode, Encode, Postgres, Type,
};
use strum::{Display, EnumDiscriminants, EnumIter, EnumString};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnumValue {
    pub name: &'static str,
    pub description: &'static str,
    pub value: i64,
}

// -- VALUTS BADGES
// -- badge for having N diffirent non zero valuts:
#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Hash,
    PartialEq,
    Eq,
    EnumIter,
    Default,
    EnumString,
    Display,
)]
pub enum ValutBadge {
    #[default]
    FirstToken,
    TokenTourist,
    TokenCollector,
    TokenTamer,
    TokenHoarder,
    TokenDiversifier,
    TokenMagnate,
    TokenOverlord,
    TokenTitan,
}

// -- TRADES BADGES
// -- badge for performing N trades:
#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Hash,
    PartialEq,
    Eq,
    EnumIter,
    Default,
    EnumString,
    Display,
)]
pub enum TradeBadge {
    #[default]
    FirstTrade,
    TradeNovice,
    TradeTornado,
    TradeTyrant,
    TradeMogul,
    TradeMagnate,
    TradeOverlord,
    TradeLegend,
    TradeTitan,
}
// -- TRANSFERS BADGES
// -- badge for performing N transfers:
#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Hash,
    PartialEq,
    Eq,
    EnumIter,
    Default,
    EnumString,
    Display,
)]
pub enum TransferBadge {
    #[default]
    FirstTransfer,
    TransferRookie,
    TransferTrooper,
    TransferCourier,
    TransferMagician,
    TransferWizard,
    TransferTerminator,
    TransferLegend,
    TransferTitan,
}
// -- ORDERS BADGES
// -- badge for performing N asks:
#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Hash,
    PartialEq,
    Eq,
    EnumIter,
    Default,
    EnumString,
    Display,
)]
pub enum MakerBadge {
    #[default]
    FirstMaker,
    MakerApprentice,
    MakerAficionado,
    MakerAvenger,
    MakerAce,
    MakerAvalanche,
    MakerOverlord,
}
// -- ORDERS BADGES
// -- badge for performing N bids:
#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Hash,
    PartialEq,
    Eq,
    EnumIter,
    Default,
    EnumString,
    Display,
)]
pub enum TakerBadge {
    #[default]
    FirstTaker,
    TakerBeginner,
    TakerBandit,
    TakerBoss,
    TakerBaron,
    TakerBandwagon,
    TakerBehemoth,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Hash,
    PartialEq,
    Eq,
    EnumDiscriminants,
    EnumString,
    Display,
)]
#[strum_discriminants(name(BadgeFamily))]
#[strum_discriminants(derive(Deserialize, EnumString, Display))]
pub enum BadgeName {
    ValutBadge(ValutBadge),
    TradeBadge(TradeBadge),
    TransferBadge(TransferBadge),
    MakerBadge(MakerBadge),
    TakerBadge(TakerBadge),
}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Badge {
    pub id: Uuid,
    pub badge_name: String,
    pub badge_family: String,
    pub badge_description: String,
    pub value: u64,
    pub created_at: DateTime<Utc>,
}
