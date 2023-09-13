use std::{collections::HashMap, env::Vars};
use sqlx::{mysql::MySqlPoolOptions, postgres::PgPoolOptions, MySqlPool, PgPool};

use crate::DATABASE_URL;

#[derive(Clone, Debug)]
struct Teliway {
    code: String,
    pool: MySqlPool,
}

#[derive(Clone, Debug)]
pub struct Databases {
    pub default: PgPool,
    pub teliways: HashMap<String, MySqlPool>,
}

impl Databases {
    pub async fn init(vars: Vars) -> Result<Self, sqlx::Error> {
        let (default_url, teliways_urls) = get_teliways_codes_from_env(vars);

        let default = Databases::init_default(default_url).await?;
        let teliways = Databases::init_teliways(teliways_urls).await;

        Ok(Databases { default, teliways })
    }

    pub async fn init_default(url: String) -> Result<PgPool, sqlx::Error> {
        PgPoolOptions::new()
            .max_connections(3)
            .connect(&url)
            .await
            //.map_err(|| Error::DatabaseError(format!("connexion {url} impossible")))
    }

    async fn init_teliways(codes: Vec<(String, String)>) -> HashMap<String, MySqlPool> {
        let futures = codes
            .into_iter()
            .map(|(code, url)| Databases::init_teliway(code, url))
            .collect::<Vec<_>>();

        let teliways = futures::future::join_all(futures).await;

        teliways
            .into_iter()
            .filter_map(Result::ok)
            .map(|t| (t.code, t.pool))
            .collect::<HashMap<_, _>>()
    }

    async fn init_teliway(code: String, url: String) -> Result<Teliway, sqlx::Error> {
        let pool = MySqlPoolOptions::new()
            .max_connections(3)
            .connect(&url)
            .await?;

        Ok(Teliway { code, pool })
    }

    pub fn get_by_code(&self, code: &str) -> Option<MySqlPool> {
        self.teliways
            .get(code)
            .cloned()
    }
}

fn get_teliways_codes_from_env(vars: Vars) -> (String, Vec<(String, String)>) {
    const PREFIX: &str = "DATABASE_";
    const SUFFIX: &str = "_URL";

    let mut default = String::new();
    let mut teliways = vec![];

    for (key, value) in vars {
        if key.starts_with(PREFIX) {
            if key == DATABASE_URL {
                default = value;
            } else if let Some(code) = key
                .strip_prefix(PREFIX)
                .and_then(|x| x.strip_suffix(SUFFIX))
            {
                teliways.push((code.to_lowercase().to_string(), value));
            }
        }
    }

    (default, teliways)
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::databases::get_teliways_codes_from_env;

    #[test]
    fn extract_codes_from_env() {
        env::set_var("DATABASE_URL", "onex");
        env::set_var("DATABASE_GTRA_URL", "tw_gtra");
        let (default, teliways) = get_teliways_codes_from_env(env::vars());

        assert_eq!(default, "onex");
        assert_eq!(teliways[0], ("gtra".to_string(), "tw_gtra".to_string()));
    }
}
