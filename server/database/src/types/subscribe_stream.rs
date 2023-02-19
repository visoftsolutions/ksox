use std::pin::Pin;

use futures::Stream;
use sqlx::{postgres::PgQueryResult, Error};

use super::NotifyTrigger;

pub struct SubscribeStream<'a, T> {
    trigger: NotifyTrigger,
    stream: Pin<Box<dyn Stream<Item = Result<T, Error>> + Send + 'a>>,
}

impl<'a, T> SubscribeStream<'a, T> {
    pub fn new(
        trigger: NotifyTrigger,
        stream: Pin<Box<dyn Stream<Item = Result<T, Error>> + Send + 'a>>,
    ) -> Self {
        SubscribeStream { trigger, stream }
    }

    pub async fn destroy(self) -> Result<PgQueryResult, Error> {
        self.trigger.destroy().await
    }
}

impl<'a, T> Stream for SubscribeStream<'a, T> {
    type Item = Result<T, Error>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.stream.as_mut().poll_next(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.stream.size_hint()
    }
}
