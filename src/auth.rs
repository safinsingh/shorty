use async_trait::async_trait;
use once_cell::unsync::Lazy;
use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome, Request},
};
use std::env::{self, VarError};

pub struct ShortyToken;

#[async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for ShortyToken {
    type Error = &'r str;

    async fn from_request(
        req: &'a Request<'r>,
    ) -> request::Outcome<Self, Self::Error> {
        let token = req
            .headers()
            .get_one("Authorization")
            .and_then(|h| h.strip_prefix("Bearer "));
        let sys_token =
            Lazy::<Result<String, VarError>>::new(|| env::var("TOKEN"));

        if token == sys_token.as_ref().ok().map(String::as_str) {
            Outcome::Success(Self)
        } else {
            Outcome::Failure((Status::Unauthorized, "Invalid API token."))
        }
    }
}
