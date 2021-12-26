use crate::mojang_api::error::ApiError;
use minreq::{Method, Request, Response, URL};

#[doc(hidden)]
pub fn fetch<U: Into<URL>>(method: Method, url: U) -> Request {
    Request::new(method, url).with_header("User-Agent", "minecraft_utils/0.1.0")
}

#[doc(hidden)]
pub fn get<U: Into<URL>>(url: U) -> Result<Response, ApiError> {
    let res = fetch(Method::Get, url).send()?;
    if res.status_code == 200 {
        Ok(res)
    } else {
        Err(ApiError::Request {
            status: res.status_code,
            reason: res.reason_phrase,
        })
    }
}
