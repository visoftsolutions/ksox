use fraction::Fraction;
use infinity::Infinity;

pub enum Value {
    Finite(Fraction),
    Infinite(Infinity),
}
