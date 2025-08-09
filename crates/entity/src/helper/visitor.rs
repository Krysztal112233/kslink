use log::{error, info};
use sea_orm::{ConnectionTrait, DbErr, EntityTrait, PaginatorTrait};

use crate::model::{prelude::*, visit_record::ActiveModel};

#[async_trait::async_trait]
pub trait VisitRecordHelper {
    async fn insert_batch<C, I>(ele: I, db: &C)
    where
        C: ConnectionTrait,
        I: IntoIterator<Item = ActiveModel> + Send,
    {
        let _ = VisitRecord::insert_many(ele)
            .exec_without_returning(db)
            .await
            .inspect_err(|err| error!("failed to insert records: {err}"))
            .inspect(|r| info!("insert {r} visit records into database"));
    }

    async fn get_count<C>(db: &C) -> Result<u64, DbErr>
    where
        C: ConnectionTrait,
    {
        VisitRecord::find().count(db).await
    }
}

impl VisitRecordHelper for VisitRecord {}
