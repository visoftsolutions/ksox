use std::str::FromStr;

use database::{
    managers::users::UsersManager,
    projections::user::User,
    sqlx::{
        types::{chrono::Utc, Uuid},
        PgPool,
    },
    types::EvmAddress,
};
use repository::{
    dbworker::{
        models::{DBWorkerRequest, DBWorkerResponse},
        DBWorker,
    },
    Repository,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();
    let database = PgPool::connect(std::env::var("DATABASE_URL").unwrap_or_default().as_str())
        .await
        .unwrap();

    let users_manager = UsersManager::new(database);
    let mut repo = Repository::<Uuid, User>::with_capacity(10, users_manager);

    // let user = User {
    //     id: Uuid::new_v4(),
    //     created_at: Utc::now(),
    //     address: EvmAddress::from_str("0x3acadfb15e991e8403d2fe3e75ee4782b88cf5b1")?,
    // };

    // repo.insert(user.id, user).await?;
    let data = repo
        .get(Uuid::from_str("75491e43-57ec-424f-81ee-702b9bfc1796")?)
        .await?;

    println!("{data:?}");
    println!("{repo:?}");

    repo.dbworker
        .dbworker_tx
        .send(DBWorkerRequest::TerminateThread)
        .await?;

    Ok(())
}

// #[tokio::main]
// async fn main() -> Result<(), anyhow::Error> {
//

//     let database = PgPool::connect(std::env::var("DATABASE_URL").unwrap_or_default().as_str())
//         .await
//         .unwrap();

//     let users_manager = UsersManager::new(database);
//     let mut db = DBWorker::<Uuid, User>::new(users_manager);
//     println!("initialized");

//     let uuid = Uuid::new_v4();

//     db.dbworker_tx
//         .send(DBWorkerRequest::Insert(User {
//             id: uuid,
//             created_at: Utc::now(),
//             address: EvmAddress::from_str("0x3acadfb15e991e8403d2fe3e75ee4782b8aaf5b2")?,
//         }))
//         .await?;

//     db.dbworker_tx
//         .send(DBWorkerRequest::Get(uuid))
//         .await?;

//     if let Some(msg) = db.dbworker_rx.recv().await {
//         match msg {
//             DBWorkerResponse::Some(v) => {
//                 println!("{:?}", v);
//             }
//             DBWorkerResponse::None => {
//                 println!("None");
//             }
//         }
//     }

//     db.dbworker_tx
//         .send(DBWorkerRequest::TerminateThread)
//         .await?;
//     Ok(())
// }
