use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
    Decode, Encode, Postgres, Type,
};
use strum::EnumIter;
use uuid::Uuid;

// const DAY: isize = 86400;
// const WEEK: isize = 7*DAY;

// // -- USERS BADGES
// // -- badge for being user for:
// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub enum UserBadge {
//     CryptoNewbie = DAY,
//     CryptoConnoisseur = 1*4*WEEK,
//     CryptoVeteran = 2*4*WEEK,
//     CryptoOverlord = 3*4*WEEK,
//     CryptonautForever = 4*4*WEEK,
// }
// -- VALUTS BADGES
// -- badge for having N diffirent non zero valuts:
#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, EnumIter)]
pub enum ValutBadge {
    TokenTourist = 1 * 10,
    TokenCollector = 2 * 10,
    TokenTamer = 3 * 10,
    TokenHoarder = 4 * 10,
    TokenDiversifier = 5 * 10,
    TokenMagnate = 6 * 10,
    TokenOverlord = 7 * 10,
    TokenTitan = 8 * 10,
}
// -- TRADES BADGES
// -- badge for performing N trades:
#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, EnumIter)]
pub enum TradeBadge {
    TradeNovice = 1 * 100,
    TradeTornado = 2 * 100,
    TradeTyrant = 3 * 100,
    TradeMogul = 4 * 100,
    TradeMagnate = 5 * 100,
    TradeOverlord = 6 * 100,
    TradeLegend = 7 * 100,
    TradeTitan = 8 * 100,
}
// -- TRANSFERS BADGES
// -- badge for performing N transfers:
#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, EnumIter)]
pub enum TransferBadge {
    TransferRookie = 1 * 100,
    TransferTrooper = 2 * 100,
    TransferCourier = 3 * 100,
    TransferMagician = 4 * 100,
    TransferWizard = 5 * 100,
    TransferTerminator = 6 * 100,
    TransferLegend = 7 * 100,
    TransferTitan = 8 * 100,
}
// -- ORDERS BADGES
// -- badge for performing N asks:
#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, EnumIter)]
pub enum AskBadge {
    AskingApprentice = 1 * 100,
    AskingAficionado = 2 * 100,
    AskAvenger = 3 * 100,
    AskingAce = 4 * 100,
    AskAvalanche = 5 * 100,
    AskOverlord = 6 * 100,
}
// -- ORDERS BADGES
// -- badge for performing N bids:
#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, EnumIter)]
pub enum BidBadge {
    BidBeginner = 1 * 100,
    BiddingBandit = 2 * 100,
    BidBoss = 3 * 100,
    BiddingBaron = 4 * 100,
    BiddingBandwagon = 5 * 100,
    BidBehemoth = 6 * 100,
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub enum BadgeName {
    // UserBadge(UserBadge),
    ValutBadge(ValutBadge),
    TradeBadge(TradeBadge),
    TransferBadge(TransferBadge),
    AskBadge(AskBadge),
    BidBadge(BidBadge),
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct Badge {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_modification_at: DateTime<Utc>,
    pub user_id: Uuid,
    pub badge_name: BadgeName,
}

impl Type<Postgres> for BadgeName {
    fn type_info() -> PgTypeInfo {
        <&str as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        <&str as Type<Postgres>>::compatible(ty)
    }
}

impl Decode<'_, Postgres> for BadgeName {
    fn decode(value: PgValueRef) -> std::result::Result<Self, sqlx::error::BoxDynError> {
        Ok(serde_json::from_str(value.as_str()?).map_err(|error| error.to_string())?)
    }
}

impl Encode<'_, Postgres> for BadgeName {
    fn produces(&self) -> Option<<Postgres as sqlx::Database>::TypeInfo> {
        Some(sqlx::postgres::PgTypeInfo::with_name("CHAR[]"))
    }

    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> sqlx::encode::IsNull {
        <&str as Encode<Postgres>>::encode_by_ref(
            &serde_json::to_string(self).unwrap_or_default().as_str(),
            buf,
        )
    }

    fn encode(
        self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        self.encode_by_ref(buf)
    }
}
