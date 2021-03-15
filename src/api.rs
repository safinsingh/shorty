use crate::{
    auth::ShortyToken,
    db::{Link, LinkRecord},
    ShortyState,
};
use anyhow::Result;
use rocket::form::Form;
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Serialize, Default)]
pub struct ApiResult<T> {
    ok: bool,
    data: Option<T>,
    err: Option<String>,
}

impl<T: Serialize + Default> ApiResult<T> {
    fn from_result(result: Result<T>) -> ApiResult<T> {
        if let Ok(r) = result {
            Self {
                ok: true,
                data: Some(r),
                err: None,
            }
        } else {
            Default::default()
        }
    }
}

#[post("/api/link/<name>", data = "<link>")]
pub async fn add_item<'r>(
    state: ShortyState<'r>,
    _token: ShortyToken,
    name: String,
    link: Form<LinkRecord>,
) -> Json<ApiResult<Link>> {
    let link = state.add_link(name, link.into_inner()).await;
    Json(ApiResult::from_result(link))
}

#[delete("/api/link/<name>")]
pub async fn delete_item<'r>(
    state: ShortyState<'r>,
    _token: ShortyToken,
    name: String,
) -> Json<ApiResult<String>> {
    let link = state.del_link(name).await;
    Json(ApiResult::from_result(link))
}
