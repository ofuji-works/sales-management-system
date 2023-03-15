use std::error::Error;

use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Pool, Sqlite, SqlitePool,
};
use std::str::FromStr;

type DbResult<T> = Result<T, Box<dyn std::error::Error>>;

pub(crate) async fn create_sqlite_pool(database_url: &str) -> DbResult<SqlitePool> {
    let connection_options =
        SqliteConnectOptions::from_str(*&database_url)?.create_if_missing(true);

    let sqlite_pool = SqlitePoolOptions::new()
        .connect_with(connection_options)
        .await?;

    Ok(sqlite_pool)
}

pub(crate) async fn migrate_database(pool: &SqlitePool) -> DbResult<()> {
    sqlx::migrate!("./database/migrations").run(pool).await?;
    Ok(())
}

pub(crate) async fn seeder() -> DbResult<()> {
    Ok(())
}

pub(crate) fn excute() -> Result<Pool<Sqlite>, Box<dyn Error>> {
    const DATABASE_DIR: &str = "sales-management-system";
    const DATABASE_FILE: &str = "db.sqlite";

    let home_dir = directories::UserDirs::new()
        .map(|dirs| dirs.home_dir().to_path_buf())
        .unwrap_or_else(|| std::env::current_dir().expect("Cannot access the current directory"));
    let database_dir = home_dir.join(DATABASE_DIR);
    let database_file = database_dir.join(DATABASE_FILE);

    let db_exists = std::fs::metadata(&database_file).is_ok();
    if !db_exists {
        std::fs::create_dir(&database_dir)?;
    }
    let database_dir_str = dunce::canonicalize(&database_dir)
        .unwrap()
        .to_string_lossy()
        .replace('\\', "/");
    let database_url = format!("sqlite://{}/{}", database_dir_str, DATABASE_FILE);

    let sqlite_pool = tauri::async_runtime::block_on(create_sqlite_pool(&database_url))?;
    if !db_exists {
        tauri::async_runtime::block_on(migrate_database(&sqlite_pool))?;
    }

    Ok(sqlite_pool)
}
