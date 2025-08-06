use entity::model::visit_record::ActiveModel;
use migration::async_trait;
use rocket::{
    request::{FromRequest, Outcome},
    Request,
};
use sea_orm::ActiveValue::Set;
use snowflake_ng::{provider::ChronoProvider, SnowflakeGenerator};

use crate::common::VisitRecordQueue;

#[derive(Debug)]
pub struct VisitRecorder;

#[async_trait::async_trait]
impl<'r> FromRequest<'r> for VisitRecorder {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let queue = request.rocket().state::<VisitRecordQueue>().unwrap();
        let snowflake = request.rocket().state::<SnowflakeGenerator>().unwrap();

        let value = {
            let Some(hash) = request.uri().path().segments().last() else {
                return Outcome::Success(VisitRecorder);
            };
            let ua = request.headers().get_one("User-Agent").unwrap_or("unknown");

            ActiveModel {
                id: Set(*snowflake.assign(&ChronoProvider).await),
                user_agent: Set(ua.to_owned()),
                ref_hash: Set(hash.to_owned()),
            }
        };

        let _ = queue.send(value);
        Outcome::Success(VisitRecorder)
    }
}
