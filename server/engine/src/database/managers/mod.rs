pub mod assets;
pub mod orders;
pub mod trades;
pub mod transfers;
pub mod valuts;

pub use assets::AssetsManager;
pub use orders::OrdersManager;
pub use trades::TradesManager;
pub use transfers::TransfersManager;
use uuid::Uuid;
pub use valuts::ValutsManager;

pub struct Id {
    id: Uuid,
}