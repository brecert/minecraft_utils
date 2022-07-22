use crate::mojang_api::error::ApiError;
use minreq::{Method, Request, Response, URL};

#[doc(hidden)]
pub fn fetch<U: Into<URL>>(method: Method, url: U) -> Request {
    Request::new(method, url).with_header(
        "User-Agent",
        concat!("minecraft_utils", env!("CARGO_PKG_VERSION")),
    )
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

#[doc(hidden)]
pub fn post<U: Into<URL>, T: serde::ser::Serialize>(
    url: U,
    body: &T,
) -> Result<Response, ApiError> {
    let res = fetch(Method::Post, url).with_json(body)?.send()?;
    if res.status_code == 200 {
        Ok(res)
    } else {
        Err(ApiError::Request {
            status: res.status_code,
            reason: res.reason_phrase,
        })
    }
}
