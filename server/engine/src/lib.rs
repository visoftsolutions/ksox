#![feature(let_chains)]

pub mod base {
    tonic::include_proto!("server.engine.base");
}

pub mod database;
pub mod matching_engine;
