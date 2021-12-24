use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use std::sync::Arc;
use serenity::prelude::TypeMapKey;
use tiberius::{AuthMethod, Config};

pub struct Database {
    pool: Pool<ConnectionManager>
}

impl TypeMapKey for Database {
    type Value = Arc<Database>;
}

impl Database {
    pub async fn new(ip: &str, port: u16, usr: &str, pwd: &str) -> Result<Self, bb8_tiberius::Error> {
        // The password is stored in a file; using secure strings is probably not going to make much of a difference.
        let mut config = Config::new();

        config.host(ip);
        config.port(port);
        config.authentication(AuthMethod::sql_server(usr, pwd));
        config.trust_cert();

        let manager = ConnectionManager::build(config)?;
        let pool = Pool::builder().max_size(8).build(manager).await?;

        Ok(Database { pool })
    }

    pub async fn get_db_version(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get().await?;
        let res = conn.simple_query("SELECT @@version")
            .await?
            .into_first_result()
            .await?
            .into_iter()
            .map(|row| {
                let val: &str = row.get(0).unwrap();
                String::from(val)
            })
            .reduce(|a, b| {format!("{}\n{}", a, b)})
            .unwrap();

        Ok(res)
    }
}