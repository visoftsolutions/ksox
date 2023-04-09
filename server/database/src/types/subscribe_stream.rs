use std::pin::Pin;
use futures::Stream;

pub type SubscribeStream<T> = Pin<Box<dyn Stream<Item = T> + Send>>;
