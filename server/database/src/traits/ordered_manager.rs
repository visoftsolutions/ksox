use std::pin::Pin;

use futures::Stream;
use sqlx::Result;

pub trait OrderedManager<T, O> {
    fn get_ordered_asc_less(
        &self,
        less_then: O,
    ) -> Pin<Box<dyn Stream<Item = Result<T>> + Send + '_>>;
    fn get_ordered_desc_less(
        &self,
        less_then: O,
    ) -> Pin<Box<dyn Stream<Item = Result<T>> + Send + '_>>;
    fn get_ordered_asc_higher(
        &self,
        higher_then: O,
    ) -> Pin<Box<dyn Stream<Item = Result<T>> + Send + '_>>;
    fn get_ordered_desc_higher(
        &self,
        higher_then: O,
    ) -> Pin<Box<dyn Stream<Item = Result<T>> + Send + '_>>;
}
