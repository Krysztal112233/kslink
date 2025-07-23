use async_trait::async_trait;
use sea_orm::{prelude::*, ActiveModelTrait, ActiveValue::*, ConnectionTrait, EntityTrait};

use crate::{
    error::Error,
    model::prelude::UrlMapping,
    model::url_mapping::{self},
};

#[async_trait]
pub trait UrlMappingHelper {
    async fn instert<C, A0, A1>(hash: A0, des: A1, db: &C) -> Result<url_mapping::Model, Error>
    where
        C: ConnectionTrait,
        A0: AsRef<str> + Send,
        A1: AsRef<str> + Send,
    {
        Ok(url_mapping::ActiveModel {
            hash: Set(hash.as_ref().to_string()),
            dest: Set(des.as_ref().to_string()),
        }
        .insert(db)
        .await?)
    }

    async fn get_from_hash<C, A>(hash: A, db: &C) -> Result<url_mapping::Model, Error>
    where
        C: ConnectionTrait,
        A: AsRef<str> + Send,
    {
        UrlMapping::find_by_id(hash.as_ref().to_string())
            .one(db)
            .await?
            .ok_or(Error::PKNotFound(hash.as_ref().to_string()))
    }

    async fn get_from_desc<C, A>(dest: A, db: &C) -> Result<url_mapping::Model, Error>
    where
        C: ConnectionTrait,
        A: AsRef<str> + Send,
    {
        UrlMapping::find()
            .filter(url_mapping::Column::Dest.eq(dest.as_ref().to_string()))
            .one(db)
            .await?
            .ok_or(Error::DestNotFound(dest.as_ref().to_string()))
    }
}
