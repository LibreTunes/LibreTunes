use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {

use lazy_static::lazy_static;
use std::env;
use diesel::{
    pg::PgConnection,
    r2d2::ConnectionManager,
    r2d2::PooledConnection,
    r2d2::Pool,
};

use diesel_migrations::{
    embed_migrations,
    EmbeddedMigrations,
    MigrationHarness,
};

// See https://leward.eu/notes-on-diesel-a-rust-orm/

// Define some types to make it easier to work with Diesel
type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConn = PooledConnection<ConnectionManager<PgConnection>>;

// Keep a global instance of the pool
lazy_static! {
    static ref DB_POOL: PgPool = init_db_pool();
}

/// Initialize the database pool
///
/// Will panic if the DATABASE_URL environment variable is not set, or if there is an error creating the pool.
///
/// # Returns
/// A database pool object, which can be used to get pooled connections
fn init_db_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    PgPool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

/// Get a pooled connection to the database
///
/// Will panic if there is an error getting a connection from the pool.
///
/// # Returns
/// A pooled connection to the database
pub fn get_db_conn() -> PgPooledConn {
    DB_POOL.get().expect("Failed to get a database connection from the pool.")
}

/// Embedded database migrations into the binary
const DB_MIGRATIONS: EmbeddedMigrations =  embed_migrations!();

/// Run any pending migrations in the database
/// Always safe to call, as it will only run migrations that have not already been run
pub fn migrate() {
    let db_con = &mut get_db_conn();
    db_con.run_pending_migrations(DB_MIGRATIONS).expect("Could not run database migrations");
}

}
}
