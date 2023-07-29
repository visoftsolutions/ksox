use std::{pin::Pin, task::Poll, time::Duration};

use futures::{Future, FutureExt};
use linked_hash_map::LinkedHashMap;
use thiserror::Error;
use tokio::time::sleep;
use tokio_stream::Stream;
use uuid::Uuid;

pub struct TimeoutQueueEntry {
    timer: Pin<Box<dyn Future<Output = ()> + Send>>,
    action: Option<Pin<Box<dyn Future<Output = ()> + Send>>>,
}

impl TimeoutQueueEntry {
    pub fn new(timeout: Duration, action: Pin<Box<dyn Future<Output = ()> + Send>>) -> Self {
        Self {
            timer: Box::pin(sleep(timeout)),
            action: Some(action),
        }
    }
}

impl Future for TimeoutQueueEntry {
    type Output = Pin<Box<dyn Future<Output = ()>>>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        match Future::poll(self.timer.as_mut(), cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(_) => match self.action.take() {
                Some(action) => Poll::Ready(action),
                None => Poll::Pending,
            },
        }
    }
}

pub struct TimeoutQueue {
    entries: LinkedHashMap<Uuid, TimeoutQueueEntry>,
    timeout: Duration,
}

impl TimeoutQueue {
    pub fn new(timeout: Duration) -> Self {
        Self {
            entries: LinkedHashMap::new(),
            timeout,
        }
    }

    pub fn insert(&mut self, action: Pin<Box<dyn Future<Output = ()> + Send>>) -> Uuid {
        let id = Uuid::new_v4();
        self.entries
            .insert(id, TimeoutQueueEntry::new(self.timeout, action));
        id
    }

    pub fn remove(&mut self, id: &Uuid) -> Option<TimeoutQueueEntry> {
        self.entries.remove(id)
    }
}

impl Stream for TimeoutQueue {
    type Item = Pin<Box<dyn Future<Output = ()>>>;
    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let poll = match self.entries.iter_mut().next() {
            Some((_, entry)) => match entry.poll_unpin(cx) {
                Poll::Ready(action) => Poll::Ready(Some(action)),
                Poll::Pending => Poll::Pending,
            },
            None => Poll::Ready(None),
        };

        if poll.is_ready() {
            self.entries.pop_front();
        }

        poll
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.entries.len();
        (len, Some(len))
    }
}

#[derive(Error, Debug)]
pub enum TimeoutQueueError {}
