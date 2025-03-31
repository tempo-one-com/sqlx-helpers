use sqlx::{
    mysql::MySqlPoolOptions, postgres::PgPoolOptions, sqlite::SqlitePoolOptions, MySqlPool, PgPool,
    SqlitePool,
};
use std::{collections::HashMap, env::Vars, str::FromStr};

use crate::DATABASE_URL;

#[derive(Debug, PartialEq, Eq)]
pub enum DatabaseType {
    Postgres,
    Sqlite,
}

impl FromStr for DatabaseType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower_s = s.to_lowercase();

        if lower_s.starts_with("postgres") {
            Ok(DatabaseType::Postgres)
        } else if lower_s.starts_with("sqlite") {
            Ok(DatabaseType::Sqlite)
        } else {
            Err(format!("Unsupported database type: {}", s))
        }
    }
}

#[derive(Clone, Debug)]
struct Teliway {
    code: String,
    pool: MySqlPool,
}

#[derive(Clone, Debug)]
pub struct Databases {
    pub teliways: HashMap<String, MySqlPool>,
}

impl Databases {
    pub async fn init_local_pg_pool(env_vars: Vars, max_connections: u32) -> Option<PgPool> {
        if let (Some(local_db_url), _) = get_database_urls_from_env(env_vars) {
            if let Ok(DatabaseType::Postgres) = DatabaseType::from_str(&local_db_url) {
                PgPoolOptions::new()
                    .max_connections(max_connections)
                    .connect(&local_db_url)
                    .await
                    .ok()
            } else {
                None
            }
        } else {
            None
        }
    }

    pub async fn init_local_sqlite_pool(
        env_vars: Vars,
        max_connections: u32,
    ) -> Option<SqlitePool> {
        if let (Some(local_db_url), _) = get_database_urls_from_env(env_vars) {
            if let Ok(DatabaseType::Sqlite) = DatabaseType::from_str(&local_db_url) {
                SqlitePoolOptions::new()
                    .max_connections(max_connections)
                    .connect(&local_db_url)
                    .await
                    .ok()
            } else {
                None
            }
        } else {
            None
        }
    }

    pub async fn init_teliway_pools(
        env_vars: Vars,
        max_connections: u32,
    ) -> HashMap<String, MySqlPool> {
        let (_, values) = get_database_urls_from_env(env_vars);

        let futures = values
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
/// Récupération des urls des bases
/// # Arguments
/// * `vars` - issu de dotenvy
/// # Returns
/// tuple avec
/// * la base défaut (optionnel) de l'application (Postgres ou Sqlite)
/// * la liste des bases teliway (vecteur vide si aucune)
fn get_database_urls_from_env(vars: Vars) -> (Option<String>, Vec<(String, String)>) {
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
    use std::{env, str::FromStr};

    use crate::databases::{get_database_urls_from_env, DatabaseType};

    #[test]
    fn extract_codes_from_env() {
        env::set_var("DATABASE_URL", "onex");
        env::set_var("DATABASE_GTRA_URL", "tw_gtra");
        let (default, teliways) = get_database_urls_from_env(env::vars());

        assert_eq!(default, Some("onex".to_string()));
        assert_eq!(teliways[0], ("gtra".to_string(), "tw_gtra".to_string()));
    }

    #[test]
    fn extract_database_type() {
        let db_type = DatabaseType::from_str("postgres://one:one/onex").unwrap();
        assert_eq!(db_type, DatabaseType::Postgres);
    }
}
