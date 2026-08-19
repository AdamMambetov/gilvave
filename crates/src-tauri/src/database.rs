use include_dir::{Dir, include_dir};
use rusqlite::{Connection, named_params};
use rusqlite_migration::Migrations;
use std::{fs, sync::LazyLock};
use tauri::{AppHandle, Manager};

/// Удобный алиас: принимает rusqlite::Error, io::Error, tauri::Error,
/// rusqlite_migration::Error и любые другие std::error::Error.
pub type BoxResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

// Макрос include_dir! встраивает всё содержимое каталога в бинарь на этапе компиляции.
// Путь $CARGO_MANIFEST_DIR указывает на каталог с Cargo.toml текущего крейта.
static MIGRATIONS_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/migrations");

static MIGRATIONS: LazyLock<Migrations<'static>> =
    LazyLock::new(|| Migrations::from_directory(&MIGRATIONS_DIR).unwrap());

pub fn initialize_database(app_handle: &AppHandle) -> BoxResult<Connection> {
    tracing::debug!("initialize_database");
    let app_dir = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| format!("app_local_data_dir: {e}"))?;
    fs::create_dir_all(&app_dir).map_err(|e| format!("create_dir_all: {e}"))?;

    let sqlite_path = app_dir.join("gilvave.sqlite");
    tracing::debug!(?sqlite_path, "открываю БД");

    let mut conn = Connection::open(&sqlite_path)
        .map_err(|e| format!("open {}: {e}", sqlite_path.display()))?;

    // PRAGMA вне транзакций (иначе no-op)
    conn.pragma_update(None, "journal_mode", "WAL")
        .map_err(|e| format!("journal_mode=WAL: {e}"))?;
    conn.pragma_update(None, "foreign_keys", "ON")
        .map_err(|e| format!("foreign_keys=ON: {e}"))?;

    MIGRATIONS
        .to_latest(&mut conn)
        .map_err(|e| format!("migrations: {e}"))?;

    Ok(conn)
}

pub fn add_item(title: &str, db: &Connection) -> Result<(), rusqlite::Error> {
    let mut statement = db.prepare("INSERT INTO items (title) VALUES (@title)")?;
    statement.execute(named_params! { "@title": title })?;

    Ok(())
}

pub fn get_all(db: &Connection) -> Result<Vec<String>, rusqlite::Error> {
    let mut statement = db.prepare("SELECT * FROM items")?;
    let mut rows = statement.query([])?;
    let mut items = Vec::new();
    while let Some(row) = rows.next()? {
        let title: String = row.get("title")?;

        items.push(title);
    }

    Ok(items)
}
