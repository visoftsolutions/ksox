#![feature(let_chains)]

pub mod base {
    tonic::include_proto!("server.engagement.base");
}

pub mod database;
pub mod engagement_engine;
