use leptos::logging::log;

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
/// Uses `DATABASE_URL` environment variable to connect to the database if set,
/// otherwise builds a connection string from other environment variables.
/// 
/// Will panic if either the `DATABASE_URL` or `POSTGRES_HOST` environment variables
/// are not set, or if there is an error creating the pool.
///
/// # Returns
/// A database pool object, which can be used to get pooled connections
fn init_db_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        // Build the database URL from environment variables
        // Construct a separate log_url to avoid logging the password
        let mut log_url = "postgres://".to_string();
        let mut url = "postgres://".to_string();

        if let Ok(user) = env::var("POSTGRES_USER") {
            url.push_str(&user);
            log_url.push_str(&user);

            if let Ok(password) = env::var("POSTGRES_PASSWORD") {
                url.push(':');
                log_url.push(':');
                url.push_str(&password);
                log_url.push_str("********");
            }

            url.push('@');
            log_url.push('@');
        }

        let host = env::var("POSTGRES_HOST").expect("DATABASE_URL or POSTGRES_HOST must be set");

        url.push_str(&host);
        log_url.push_str(&host);

        if let Ok(port) = env::var("POSTGRES_PORT") {
            url.push(':');
            url.push_str(&port);
            log_url.push(':');
            log_url.push_str(&port);
        }

        if let Ok(dbname) = env::var("POSTGRES_DB") {
            url.push('/');
            url.push_str(&dbname);
            log_url.push('/');
            log_url.push_str(&dbname);
        }

        log!("Connecting to database: {}", log_url);
        url
    });

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
