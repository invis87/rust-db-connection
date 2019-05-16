extern crate connect_to_postgres;
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate slog;

use self::connect_to_postgres::*;
use self::models::*;
use self::logging::setup_logging;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;

fn get_connection(pool: &Pool<ConnectionManager<PgConnection>>) -> Option<DB> {
    pool.get().ok().map(|conn| DB { conn })

}

pub struct DB { conn: PooledConnection<ConnectionManager<PgConnection>> }

impl DB {
    pub fn conn(&self) -> &PgConnection {
        &self.conn
    }
}

fn main() {
    let log = setup_logging();
    use connect_to_postgres::schema::posts::dsl::*;

    let connection_pool = create_db_pool();

    loop {
        let pooled_connection_opt = get_connection(&connection_pool);
        if pooled_connection_opt.is_none() {
            info!(log, "fail to get connection");
        } else {
            let pooled_connection = pooled_connection_opt.unwrap();
            let connection: &PgConnection = pooled_connection.conn();

            let results = posts.filter(published.eq(true))
                .limit(5)
                .load::<Post>(connection)
                .expect("Error loading posts");

            info!(log, "Get {} posts", results.len());
        }
        std::thread::sleep_ms(1000);
    }

}