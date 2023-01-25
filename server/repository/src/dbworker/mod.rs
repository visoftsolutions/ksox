pub mod models;
use std::{
    io::{Error, ErrorKind},
    sync::mpsc::{channel, Receiver, Sender},
    thread::{self, JoinHandle},
};

use database::traits::manager::Manager;
use models::{DBWorkerRequest, DBWorkerResponse};
use sqlx::PgPool;

#[derive(Debug)]
pub struct DBWorker<K, V>
where
    K: Send + 'static,
    V: Send + 'static,
{
    pub dbworker_tx: Sender<DBWorkerRequest<K, V>>,
    pub dbworker_rx: Receiver<DBWorkerResponse<V>>,
    dbworker_handle: JoinHandle<Result<(), anyhow::Error>>,
}

impl<K, V> DBWorker<K, V>
where
    K: Send + 'static,
    V: Send + 'static,
{
    pub fn new<M>(db_manager: M) -> Self
    where
        M: Manager<V> + Send + 'static,
    {
        let (req_tx, req_rx) = channel::<DBWorkerRequest<K, V>>();
        let (res_tx, res_rx) = channel::<DBWorkerResponse<V>>();

        Self {
            dbworker_tx: req_tx,
            dbworker_rx: res_rx,
            dbworker_handle: thread::spawn(move || db_worker(res_tx, req_rx, db_manager)),
        }
    }

    pub fn drop(self) -> Result<(), anyhow::Error> {
        self.dbworker_handle
            .join()
            .map_err(|_| anyhow::Error::from(Error::from(ErrorKind::WouldBlock)))?
    }
}

#[tokio::main]
async fn db_worker<K, V>(
    res_tx: Sender<DBWorkerResponse<V>>,
    req_rx: Receiver<DBWorkerRequest<K, V>>,
    db_manager: impl Manager<V>,
) -> Result<(), anyhow::Error> {
    let database = PgPool::connect(std::env::var("DATABASE_URL").unwrap_or_default().as_str())
        .await
        .unwrap();
    println!("hello dbworker");
    Ok(())
}
