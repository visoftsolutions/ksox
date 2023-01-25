use std::{borrow::Borrow, collections::hash_map::RandomState, hash::Hash, marker::Sync};

use database::traits::manager::Manager;
use linked_hash_map::LinkedHashMap;

use crate::dbworker::{
    models::{DBWorkerRequest, DBWorkerResponse},
    DBWorker,
};

#[derive(Debug)]
pub struct Repository<K, V>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    capacity: usize,
    dbworker: DBWorker<K, V>,
    store: LinkedHashMap<K, V, RandomState>,
}

impl<K, V> Repository<K, V>
where
    K: Hash + Eq + Send + Sync + Clone + 'static,
    V: Send + Sync + Clone + 'static,
{
    pub fn new<M>(capacity: usize, db_manager: M) -> Self
    where
        M: Manager<V> + Send + 'static,
    {
        Self {
            capacity,
            dbworker: DBWorker::new(db_manager),
            store: LinkedHashMap::new(),
        }
    }

    pub fn with_capacity<M>(capacity: usize, db_manager: M) -> Self
    where
        M: Manager<V> + Send + 'static,
    {
        Self {
            capacity,
            dbworker: DBWorker::new(db_manager),
            store: LinkedHashMap::with_capacity(capacity),
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.store.len()
    }

    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }

    pub fn clear(&mut self) {
        self.store.clear();
    }

    pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> Result<bool, anyhow::Error>
    where
        K: Borrow<Q>,
        Q: Eq + Hash,
    {
        Ok(self.store.contains_key(k))
    }

    pub fn get(&mut self, k: K) -> Result<Option<V>, anyhow::Error> {
        // If value is found, it is moved to the end of the list.
        Ok(match self.store.get_refresh(&k) {
            Some(v) => Some(v.clone()),
            None => {
                self.dbworker.dbworker_tx.send(DBWorkerRequest::Get(k))?;
                match self.dbworker.dbworker_rx.recv()? {
                    DBWorkerResponse::Some(v) => Some(v),
                    DBWorkerResponse::None => None,
                }
            }
        })
    }

    pub fn insert(&mut self, k: K, v: V) -> Result<(), anyhow::Error> {
        // Whatever if value is found or not it pushes element to the end of the list
        self.store.insert(k.clone(), v.clone());
        if self.store.len() > self.capacity {
            self.store.pop_front();
        }
        Ok(self
            .dbworker
            .dbworker_tx
            .send(DBWorkerRequest::Insert(k, v))?)
    }

    pub fn drop(mut self) -> Result<(), anyhow::Error> {
        self.clear();
        self.dbworker.drop()
    }
}
