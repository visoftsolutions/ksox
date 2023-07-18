pub mod asset;
pub mod deposit;
pub mod user;
pub mod valut;
pub mod withdraw;

pub trait Confirmable {
    fn set(&mut self, confirmations: usize);
    fn is_confirmed(&self) -> bool;
}