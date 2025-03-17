use sqlx::{
    mysql::MySqlPoolOptions, postgres::PgPoolOptions, sqlite::SqlitePoolOptions, MySqlPool, PgPool,
    Pool, Postgres, Sqlite, SqlitePool,
};
use std::{collections::HashMap, env::Vars, str::FromStr};

use crate::DATABASE_URL;

pub enum DatabaseType {
    Postgres,
    Sqlite,
}

impl FromStr for DatabaseType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "postgres" | "postgresql" | "pg" => Ok(DatabaseType::Postgres),
            "sqlite" | "sqlite3" => Ok(DatabaseType::Sqlite),
            _ => Err(format!("Unsupported database type: {}", s)),
        }
    }
}

#[derive(Clone, Debug)]
pub enum DbPool {
    Pg(Pool<Postgres>),
    Sqlite(Pool<Sqlite>),
}

#[derive(Clone, Debug)]
struct Teliway {
    code: String,
    pool: MySqlPool,
}

#[derive(Clone, Debug)]
pub struct Databases {
    pub default: Option<DbPool>,
    pub teliways: HashMap<String, MySqlPool>,
}

impl Databases {
    pub async fn init(
        vars: Vars,
        default_max_connections: u32,
        teliway_max_connections: u32,
    ) -> Result<Self, sqlx::Error> {
        let (default_url, teliways_urls) = get_teliways_codes_from_env(vars);

        let default = match default_url {
            Some(url) => {
                let db_type = DatabaseType::from_str(&url).map_err(|err_msg| {
                    sqlx::Error::Configuration(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Type de base inconnu: {}", err_msg),
                    )))
                })?;
                Some(Databases::init_default(db_type, url, default_max_connections).await?)
            }
            _ => None,
        };

        let teliways = Databases::init_teliways(teliways_urls, teliway_max_connections).await;

        Ok(Databases { default, teliways })
    }

    // pub async fn init_default(url: String, max_connections: u32) -> Result<PgPool, sqlx::Error> {
    //     PgPoolOptions::new()
    //         .max_connections(max_connections)
    //         .connect(&url)
    //         .await
    // }

    // pub async fn init_default(url: String, max_connections: u32) -> Result<AnyPool, sqlx::Error> {
    //     AnyPoolOptions::new()
    //         .max_connections(max_connections)
    //         .connect(&url)
    //         .await
    // }

    pub async fn init_default_pg_pool(
        url: String,
        max_connections: u32,
    ) -> Result<PgPool, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(&url)
            .await?;

        Ok(pool)
    }

    pub async fn init_default_sqlite_pool(
        url: String,
        max_connections: u32,
    ) -> Result<SqlitePool, sqlx::Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(max_connections)
            .connect(&url)
            .await?;

        Ok(pool)
    }

    async fn init_teliways(
        codes: Vec<(String, String)>,
        max_connections: u32,
    ) -> HashMap<String, MySqlPool> {
        let futures = codes
            .into_iter()
            .map(|(code, url)| Databases::init_teliway(code, url, max_connections))
            .collect::<Vec<_>>();

        let teliways = futures::future::join_all(futures).await;

        teliways
            .into_iter()
            .filter_map(Result::ok)
            .map(|t| (t.code, t.pool))
            .collect::<HashMap<_, _>>()
    }

    async fn init_teliway(
        code: String,
        url: String,
        max_connections: u32,
    ) -> Result<Teliway, sqlx::Error> {
        let pool = MySqlPoolOptions::new()
            .max_connections(max_connections)
            .connect(&url)
            .await?;

        Ok(Teliway { code, pool })
    }

    pub fn get_by_code(&self, code: &str) -> Option<MySqlPool> {
        self.teliways.get(code).cloned()
    }
}

fn get_teliways_codes_from_env(vars: Vars) -> (Option<String>, Vec<(String, String)>) {
    const PREFIX: &str = "DATABASE_";
    const SUFFIX: &str = "_URL";

    let mut default = None;
    let mut teliways = vec![];

    for (key, value) in vars {
        if key.starts_with(PREFIX) {
            if key == DATABASE_URL {
                default = Some(value);
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

        assert_eq!(default, Some("onex".to_string()));
        assert_eq!(teliways[0], ("gtra".to_string(), "tw_gtra".to_string()));
    }
}
