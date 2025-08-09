use async_trait::async_trait;
use sea_orm::{prelude::*, ActiveModelTrait, ActiveValue::*, ConnectionTrait, EntityTrait};
use serde_json::json;

use crate::{
    model::prelude::UrlMapping,
    model::url_mapping::{self},
};

#[async_trait]
pub trait UrlMappingHelper {
    async fn create_short<C, A0, A1>(hash: A0, des: A1, db: &C) -> Result<url_mapping::Model, DbErr>
    where
        C: ConnectionTrait,
        A0: AsRef<str> + Send,
        A1: AsRef<str> + Send,
    {
        Ok(url_mapping::ActiveModel {
            hash: Set(hash.as_ref().to_string()),
            dest: Set(des.as_ref().to_string()),
            trimmed: Set(json!({})),
        }
        .insert(db)
        .await?)
    }

    async fn get_by_hash<C, A>(hash: A, db: &C) -> Result<url_mapping::Model, DbErr>
    where
        C: ConnectionTrait,
        A: AsRef<str> + Send,
    {
        UrlMapping::find_by_id(hash.as_ref().to_string())
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(hash.as_ref().to_string()))
    }

    async fn get_by_desc<C, A>(dest: A, db: &C) -> Result<url_mapping::Model, DbErr>
    where
        C: ConnectionTrait,
        A: AsRef<str> + Send,
    {
        UrlMapping::find()
            .filter(url_mapping::Column::Dest.eq(dest.as_ref().to_string()))
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(dest.as_ref().to_string()))
    }

    async fn get_count<C>(db: &C) -> Result<u64, DbErr>
    where
        C: ConnectionTrait,
    {
        Ok(UrlMapping::find().count(db).await?)
    }
}

impl UrlMappingHelper for UrlMapping {}
