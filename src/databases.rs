use sqlx::{
    mysql::MySqlPoolOptions, postgres::PgPoolOptions, sqlite::SqlitePoolOptions, MySqlPool, PgPool,
    SqlitePool,
};
use std::{collections::HashMap, env::Vars, str::FromStr};

use crate::DATABASE_URL;

#[derive(Debug, PartialEq, Eq)]
pub enum DatabaseType {
    Postgres,
    MySql,
    Sqlite,
}

impl FromStr for DatabaseType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        if lower.starts_with("postgres") {
            Ok(DatabaseType::Postgres)
        } else if lower.starts_with("mysql") {
            Ok(DatabaseType::MySql)
        } else if lower.starts_with("sqlite") {
            Ok(DatabaseType::Sqlite)
        } else {
            Err(format!("Unsupported database type: {}", s))
        }
    }
}

#[derive(Clone, Debug)]
pub struct Databases {
    pub teliways: HashMap<String, MySqlPool>,
}

impl Databases {
    pub async fn init_local_pg_pool_from_vars(env_vars: Vars, max_connections: u32) -> Option<PgPool> {
        let (url, _) = get_database_urls_from_env(env_vars);
        connect_pg(&url?, max_connections).await
    }

    pub async fn init_local_pg_pool_from_url(url: &str, max_connections: u32) -> Option<PgPool> {
        connect_pg(url, max_connections).await
    }

    pub async fn init_local_sqlite_pool_from_vars(
        env_vars: Vars,
        max_connections: u32,
    ) -> Option<SqlitePool> {
        let (url, _) = get_database_urls_from_env(env_vars);
        connect_sqlite(&url?, max_connections).await
    }

    pub async fn init_mysql_pools_from_vars(
        env_vars: Vars,
        max_connections: u32,
    ) -> HashMap<String, MySqlPool> {
        let (_, values) = get_database_urls_from_env(env_vars);

        futures::future::join_all(values.into_iter().map(|(code, url)| async move {
            connect_mysql(&url, max_connections).await.map(|pool| (code, pool))
        }))
        .await
        .into_iter()
        .flatten()
        .collect()
    }

    pub async fn init_mysql_pool_from_url(url: &str, max_connections: u32) -> Option<MySqlPool> {
        connect_mysql(url, max_connections).await
    }

    pub fn get_by_code(&self, code: &str) -> Option<MySqlPool> {
        self.teliways.get(code).cloned()
    }
}

async fn connect_pg(url: &str, max_connections: u32) -> Option<PgPool> {
    if DatabaseType::from_str(url) == Ok(DatabaseType::Postgres) {
        PgPoolOptions::new().max_connections(max_connections).connect(url).await.ok()
    } else {
        None
    }
}

async fn connect_mysql(url: &str, max_connections: u32) -> Option<MySqlPool> {
    if DatabaseType::from_str(url) == Ok(DatabaseType::MySql) {
        MySqlPoolOptions::new().max_connections(max_connections).connect(url).await.ok()
    } else {
        None
    }
}

async fn connect_sqlite(url: &str, max_connections: u32) -> Option<SqlitePool> {
    if DatabaseType::from_str(url) == Ok(DatabaseType::Sqlite) {
        SqlitePoolOptions::new().max_connections(max_connections).connect(url).await.ok()
    } else {
        None
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
                teliways.push((code.to_lowercase(), value));
            }
        }
    }

    (default, teliways)
}

#[cfg(test)]
mod tests {
    use std::{env, str::FromStr};
    use tokio;

    use crate::databases::{get_database_urls_from_env, DatabaseType};

    use super::Databases;

    #[test]
    fn extract_codes_from_env() {
        env::set_var("DATABASE_URL", "postgres://one:one@localhost/access");
        env::set_var("DATABASE_GTRA_URL", "tw_gtra");
        let (default, teliways) = get_database_urls_from_env(env::vars());

        assert_eq!(
            default,
            Some("postgres://one:one@localhost/access".to_string())
        );
        assert_eq!(teliways[0], ("gtra".to_string(), "tw_gtra".to_string()));
    }

    #[test]
    fn extract_database_type() {
        let db_type = DatabaseType::from_str("postgres://one:one/onex").unwrap();
        assert_eq!(db_type, DatabaseType::Postgres);
    }

    #[tokio::test]
    async fn init_mysql_pool() {
        let pool =
            Databases::init_mysql_pool_from_url("mysql://root@localhost/tw_gtra", 1).await;
        assert!(pool.is_some());
    }

    #[tokio::test]
    async fn init_teliway_pool_from_env() {
        env::set_var("DATABASE_GTRA_URL", "mysql://root@localhost/tw_gtra");
        let teliway_pools = Databases::init_mysql_pools_from_vars(env::vars(), 1).await;
        let gtra_pool = teliway_pools.get("gtra");

        assert!(gtra_pool.is_some());
        if let Some(pool) = gtra_pool {
            let count: i64 = sqlx::query_scalar("select count(*) from tiers")
                .fetch_one(pool)
                .await
                .unwrap();
            assert!(count > 0);
        };
    }

    #[tokio::test]
    async fn init_postgres_pool_from_env() {
        env::set_var("DATABASE_URL", "postgres://one:one@localhost/access");
        let pool = Databases::init_local_pg_pool_from_vars(env::vars(), 1).await;

        assert!(pool.is_some());
        if let Some(pool) = pool {
            let count: i64 = sqlx::query_scalar("select count(*) from client")
                .fetch_one(&pool)
                .await
                .unwrap();
            assert!(count > 0);
        };
    }
}
