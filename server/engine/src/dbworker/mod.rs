pub mod models;
use std::{
    fmt::Debug,
    thread::{self, JoinHandle},
};

use database::{
    sqlx::{error::Error, types::Uuid},
    traits::manager::Manager,
};
use models::{DBWorkerRequest, DBWorkerResponse};
use tokio::sync::mpsc::{channel, Receiver, Sender};

#[derive(Debug)]
pub struct DBWorker<K, V>
where
    K: Send + Sync + Debug + 'static,
    V: Send + Sync + Debug + 'static,
{
    pub dbworker_tx: Sender<DBWorkerRequest<K, V>>,
    pub dbworker_rx: Receiver<DBWorkerResponse<V>>,
    dbworker_handle: Option<JoinHandle<Result<(), anyhow::Error>>>,
}

impl<K, V> DBWorker<K, V>
where
    K: Send + Sync + Debug + 'static,
    V: Send + Sync + Debug + 'static,
    Uuid: From<K>,
{
    pub fn new<M>(db_manager: M) -> Self
    where
        M: Manager<V> + Send + 'static,
    {
        let (req_tx, req_rx) = channel::<DBWorkerRequest<K, V>>(100); //TODO change capacity !!
        let (res_tx, res_rx) = channel::<DBWorkerResponse<V>>(100); //TODO change capacity !!

        Self {
            dbworker_tx: req_tx,
            dbworker_rx: res_rx,
            dbworker_handle: Some(thread::spawn(move || db_worker(res_tx, req_rx, db_manager))),
        }
    }
}

impl<K, V> Drop for DBWorker<K, V>
where
    K: Send + Sync + Debug + 'static,
    V: Send + Sync + Debug + 'static,
{
    fn drop(&mut self) {
        if let Some(thread) = self.dbworker_handle.take() {
            tracing::info!("{:?}", thread.join().unwrap());
        }
    }
}

#[tokio::main]
async fn db_worker<K, V>(
    res_tx: Sender<DBWorkerResponse<V>>,
    mut req_rx: Receiver<DBWorkerRequest<K, V>>,
    db_manager: impl Manager<V>,
) -> Result<(), anyhow::Error>
where
    K: Send + Sync + Debug + 'static,
    V: Send + Sync + Debug + 'static,
    Uuid: From<K>,
{
    tracing::info!("db_worker started");

    while let Some(msg) = req_rx.recv().await {
        match msg {
            DBWorkerRequest::Insert(v) => {
                tracing::info!("DBWorkerRequest::Insert");
                if let Err(err) = db_manager.insert(v).await {
                    tracing::error!("{err}");
                }
            }
            DBWorkerRequest::Get(k) => {
                tracing::info!("DBWorkerRequest::Get");
                match db_manager.get_by_id(Uuid::from(k)).await {
                    Ok(v) => {
                        res_tx.send(DBWorkerResponse::Some(v)).await?;
                    }
                    Err(err) => match err {
                        Error::RowNotFound => {
                            res_tx.send(DBWorkerResponse::None).await?;
                        }
                        _ => {
                            tracing::error!("{err}");
                            res_tx.send(DBWorkerResponse::None).await?;
                        }
                    },
                };
            }
            DBWorkerRequest::TerminateThread => {
                tracing::info!("DBWorkerRequest::TerminateThread");
                break;
            }
        }
    }

    Ok(())
}
