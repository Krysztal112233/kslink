use std::{collections::HashMap, io::Cursor};

use educe::Educe;
use rocket::{
    http::{ContentType, Status},
    response::Responder,
    Response,
};
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Educe)]
#[educe(Default)]
pub struct CommonResponse {
    #[educe(Default = 200)]
    code: u16,

    #[educe(Default = None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    msg: Option<String>,

    #[serde(skip_serializing_if = "HashMap::is_empty", flatten)]
    data: HashMap<String, serde_json::Value>,
}

#[allow(unused)]
impl CommonResponse {
    pub fn new(code: u16) -> Self {
        Self {
            code,
            ..Default::default()
        }
    }

    pub fn with_msg(code: u16, msg: String) -> Self {
        Self {
            code,
            msg: Some(msg),
            ..Default::default()
        }
    }

    pub fn append<T>(mut self, key: T, value: serde_json::Value) -> Self
    where
        T: ToString,
    {
        self.data.insert(key.to_string(), value);
        Self { ..self }
    }

    pub fn append_all(mut self, all: HashMap<String, serde_json::Value>) -> Self {
        self.data.extend(all);
        Self { ..self }
    }

    pub fn into_reponse<'a>(self) -> rocket::response::Result<'a> {
        let status = Status::new(self.code);
        let body = serde_json::to_string(&self).map_err(|_| Status::new(500))?;

        let mut response = Response::new();
        response.set_sized_body(body.len(), Cursor::new(body));
        response.set_status(status);
        response.set_header(ContentType::JSON);

        Ok(response)
    }
}

impl From<Status> for CommonResponse {
    fn from(value: Status) -> Self {
        Self {
            code: value.code,
            msg: Some(value.to_string()),
            ..Default::default()
        }
    }
}

impl From<Error> for CommonResponse {
    fn from(value: Error) -> Self {
        CommonResponse::with_msg(value.clone().into(), value.to_string())
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for CommonResponse {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        let status = Status::new(self.code);
        let body = serde_json::to_string(&self).map_err(|_| Status::new(500))?;

        Response::build_from(body.respond_to(request)?)
            .status(status)
            .header(ContentType::JSON)
            .ok()
    }
}

pub enum Either<T0, T1> {
    Left(T0),
    Right(T1),
}

impl<'r, 'o: 'r, T0, T1> Responder<'r, 'o> for Either<T0, T1>
where
    T0: Responder<'r, 'o>,
    T1: Responder<'r, 'o>,
{
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        match self {
            Either::Left(a) => a.respond_to(request),
            Either::Right(b) => b.respond_to(request),
        }
    }
}
