use actix_web::{
    dev::{self, ServiceResponse},
    http::header,
    middleware::ErrorHandlerResponse,
    HttpResponse,
};

use crate::common::response::CommonResponse;

pub fn default_error_handler<B>(
    res: dev::ServiceResponse<B>,
) -> actix_web::Result<ErrorHandlerResponse<B>> {
    let (req, res) = res.into_parts();

    let mut res = HttpResponse::new(res.status()).set_body(
        serde_json::to_string(
            &res.status()
                .canonical_reason()
                .map(|msg| CommonResponse::with_msg(res.status().as_u16(), msg.to_string()))
                .unwrap_or(CommonResponse::new(res.status().as_u16())),
        )
        .unwrap(),
    );
    res.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    let res = ServiceResponse::new(req, res)
        .map_into_boxed_body()
        .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(res))
}
