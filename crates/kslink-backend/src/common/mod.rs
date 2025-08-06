use entity::model::visit_record;
use rocket::tokio::sync::mpsc;

pub mod fairing;
pub mod guard;
pub mod hasher;
pub mod request;
pub mod response;

pub type VisitRecordQueue = mpsc::UnboundedSender<visit_record::ActiveModel>;
