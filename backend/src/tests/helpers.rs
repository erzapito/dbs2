#[cfg(test)]
pub mod tests {

    use crate::config::Config;
    use crate::database::{DatabaseConnection, init_pool, Pool};
    use actix_web::{ web::Data };
    use diesel::sqlite::SqliteConnection;

    lazy_static! {
        pub static ref TEST_CONFIG: Config = get_test_config();
    }

    fn get_test_config() -> Config {
        Config {
          database: DatabaseConnection::Sqlite,
          database_url: ":memory:".to_string(),
          server: "127.0.0.1:3000".to_string(),
        }
    }

    /// Returns a r2d2 Pooled Connection to be used in tests
    pub fn init_test_pool() -> Pool<SqliteConnection> {
        let pool = init_pool::<SqliteConnection>(TEST_CONFIG.clone()).unwrap();
        let conn = pool.get().unwrap();
        embed_migrations!("./migrations/");
        embedded_migrations::run(&conn).unwrap();
//        embedded_migrations::run_with_output(&conn, &mut std::io::stdout()).unwrap();
        pool
    }

    /// Returns a r2d2 Pooled Connection wrappedn in Actix Application Data
    pub fn get_data_pool() -> Data<Pool<SqliteConnection>> {
        Data::new( init_test_pool() )
    }

}
