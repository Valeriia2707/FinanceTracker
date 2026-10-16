pub mod models;
pub mod repository;
pub mod nbu;

use rusqlite::{Connection, Result};
use std::fs;
use std::path::PathBuf;

fn get_db_path() -> PathBuf {
    let mut path = dirs::data_dir().expect("Не вдалося знайти системну папку даних");
    path.push("FinanceTracker");
    fs::create_dir_all(&path).ok();
    path.push("app_data.db");
    path
}

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open(get_db_path())?;

    // Увімкнення перевірки зовнішніх ключів у SQLite
    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    // 1. Категорії витрат
    conn.execute(
        "CREATE TABLE IF NOT EXISTS categories (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL
        )",
        [],
    )?;

    // 2. Витрати (мають зовнішній ключ на категорію)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS expenses (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            category_id INTEGER NOT NULL,
            date TEXT NOT NULL,                 -- Формат YYYY-MM-DD
            original_amount REAL NOT NULL,
            currency TEXT NOT NULL,            -- USD, EUR, PLN, GBP, UAH
            exchange_rate REAL NOT NULL,
            converted_amount_uah REAL NOT NULL,
            FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE RESTRICT
        )",
        [],
    )?;

    // 3. Доходи (без прив'язки до категорії)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS incomes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            date TEXT NOT NULL,                 -- Формат YYYY-MM-DD
            original_amount REAL NOT NULL,
            currency TEXT NOT NULL,            -- USD, EUR, PLN, GBP, UAH
            exchange_rate REAL NOT NULL,
            converted_amount_uah REAL NOT NULL
        )",
        [],
    )?;

    // 4. Історія курсів валют НБУ по днях
    conn.execute(
        "CREATE TABLE IF NOT EXISTS exchange_rates (
            currency TEXT NOT NULL,
            date TEXT NOT NULL,                 -- Формат YYYY-MM-DD
            rate REAL NOT NULL,
            PRIMARY KEY (currency, date)
        )",
        [],
    )?;

    // Базові категорії, якщо таблиця порожня
    conn.execute(
        "INSERT OR IGNORE INTO categories (name) VALUES
            ('Продукти'),
            ('Кафе та ресторани'),
            ('Транспорт'),
            ('Розваги'),
            ('Комунальні послуги'),
            ('Покупки / Техніка'),
            ('Здоров''я'),
            ('Інше')",
        [],
    )?;

    Ok(conn)
}