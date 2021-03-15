use async_trait::async_trait;
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    Request, Response,
};

pub struct Attribution;

#[async_trait]
impl Fairing for Attribution {
    fn info(&self) -> Info {
        Info {
            name: "Attribution",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(
        &self,
        _req: &'r Request<'_>,
        res: &mut Response<'r>,
    ) {
        res.set_header(Header::new(
            "X-Powered-By",
            "shorty (https://github.com/cjdenio/shorty)",
        ));
    }
}
