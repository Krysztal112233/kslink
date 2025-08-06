use std::time::Duration;

use entity::{
    helper::visitor::VisitRecordHelper,
    model::{prelude::*, visit_record},
};
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    tokio::{self, sync::mpsc, time},
    Request, Response, Shutdown,
};
use sea_orm::DatabaseConnection;

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

pub struct VisitQueueConsumer;

impl VisitQueueConsumer {
    pub async fn run(
        conn: DatabaseConnection,
        mut rx: mpsc::UnboundedReceiver<visit_record::ActiveModel>,
        shutdown: Shutdown,
    ) {
        let mut buffer = Vec::with_capacity(1024);
        let mut tick = time::interval(Duration::from_secs(1));
        tick.tick().await;

        loop {
            tokio::select! {
                _ = shutdown.clone() => {
                    if !buffer.is_empty() { VisitRecord::insert_batch(std::mem::take(&mut buffer), &conn).await }
                    break;
                }
                _ = tick.tick() => {
                    if !buffer.is_empty() { VisitRecord::insert_batch(std::mem::take(&mut buffer), &conn).await }
                }
                item = rx.recv() => match item {
                    Some(model) => {
                        buffer.push(model);
                        if buffer.len() == 1024 { VisitRecord::insert_batch(std::mem::take(&mut buffer), &conn).await }
                    }
                    None => {
                        if !buffer.is_empty() { VisitRecord::insert_batch(std::mem::take(&mut buffer), &conn).await }
                        break;
                    }
                }
            }
        }
    }
}
