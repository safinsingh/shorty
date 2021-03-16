use anyhow::Result;
use r2d2::Pool;
use redis::{Client, Commands};
use rocket::form::FromForm;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Link {
    name: String,
    link: LinkRecord,
}

#[derive(Serialize, Deserialize, Debug, FromForm, Default)]
pub struct LinkRecord {
    pub url: String,
}

pub struct ShortyDb {
    pool: Pool<Client>,
}

impl ShortyDb {
    pub fn new(pool: Pool<Client>) -> Self { ShortyDb { pool } }

    pub async fn add_link(
        &self,
        name: String,
        link: LinkRecord,
    ) -> Result<Link> {
        let mut conn = self.pool.get()?;
        let link_json = serde_json::to_string(&link)?;

        conn.set(format!("link:{}", name), link_json)?;
        Ok(Link { name, link })
    }

    pub async fn del_link(&self, name: String) -> Result<String> {
        let mut conn = self.pool.get()?;
        conn.del(format!("link:{}", name))?;

        Ok(name)
    }

    pub async fn get_link(&self, name: String) -> Result<LinkRecord> {
        let mut conn = self.pool.get()?;
        let link = conn.get::<_, String>(format!("link:{}", name))?;

        serde_json::from_str::<LinkRecord>(&link).map_err(Into::into)
    }
}
