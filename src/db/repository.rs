use rusqlite::{params, Connection, Result};
use rusqlite::OptionalExtension;
use super::models::Category;
use super::models::{Category, ExpenseItem, NewExpense, Income, ExchangeRate};

pub fn add_category(conn: &Connection, name: &str) -> Result<i64> {
    conn.execute(
        "INSERT INTO categories (name) VALUES (?1)",
        params![name],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn get_all_categories(conn: &Connection) -> Result<Vec<Category>> {
    let mut stmt = conn.prepare("SELECT id, name FROM categories ORDER BY name ASC")?;
    let rows = stmt.query_map([], |row| {
        Ok(Category {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    let mut categories = Vec::new();
    for category in rows {
        categories.push(category?);
    }
    Ok(categories)
}

pub fn delete_category(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM categories WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn add_expense(conn: &Connection, expense: &NewExpense) -> Result<i64> {
    conn.execute(
        "INSERT INTO expenses (title, category_id, date, original_amount, currency, exchange_rate, converted_amount_uah) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            expense.title,
            expense.category_id,
            expense.date,
            expense.original_amount,
            expense.currency,
            expense.exchange_rate,
            expense.converted_amount_uah
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn get_all_expenses(conn: &Connection) -> Result<Vec<ExpenseItem>> {
    let mut stmt = conn.prepare(
        "SELECT 
    e.id,
    e.title,
    c.name,
    e.date,
    e.original_amount,
    e.currency,
    e.exchange_rate,
    e.converted_amount_uah
    FROM expenses e
    JOIN categories c ON e.category_id = c.id
    ORDER BY e.date DESC, e.id DESC",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(ExpenseItem {
            id: row.get(0)?,
            title: row.get(1)?,
            category_name: row.get(2)?,
            date: row.get(3)?,
            original_amount: row.get(4)?,
            currency: row.get(5)?,
            exchange_rate: row.get(6)?,
            converted_amount_uah: row.get(7)?,
        })
    })?;

    let mut expenses = Vec::new();
    for expense in rows {
        expenses.push(expense?);
    }
    Ok(expenses)
}

pub fn add_income(conn: &Connection, income: &Income) -> Result<i64> {
    conn.execute(
        "INSERT INTO incomes (title, date, original_amount, currency, exchange_rate, converted_amount_uah) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            income.title,
            income.date,
            income.original_amount,
            income.currency,
            income.exchange_rate,
            income.converted_amount_uah
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn get_all_incomes(conn: &Connection) -> Result<Vec<Income>> {
    let mut stmt = conn.prepare(
        "SELECT 
    id,
    title,
    date,
    original_amount,
    currency,
    exchange_rate,
    converted_amount_uah
    FROM incomes
    ORDER BY date DESC, id DESC",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(Income {
            id: row.get(0)?,
            title: row.get(1)?,
            date: row.get(2)?,
            original_amount: row.get(3)?,
            currency: row.get(4)?,
            exchange_rate: row.get(5)?,
            converted_amount_uah: row.get(6)?,
        })
    })?;

    let mut incomes = Vec::new();
    for income in rows {
        incomes.push(income?);
    }
    Ok(incomes)
}

pub fn get_total_balance(conn: &Connection) -> Result<f64> {
    let total_income: f64 = conn.query_row(
        "SELECT COALESCE(SUM(converted_amount_uah), 0.0) FROM incomes",
        [],
        |row| row.get(0),
    )?;
    let total_expense: f64 = conn.query_row(
        "SELECT COALESCE(SUM(converted_amount_uah), 0.0) FROM expenses",
        [],
        |row| row.get(0),
    )?;
    Ok(total_income - total_expense)
}

pub fn get_exchange_rate(conn: &Connection, currency: &str, date: &str) -> Result<Option<f64>> {
    let mut stmt = conn.prepare(
        "SELECT rate FROM exchange_rates WHERE currency = ?1 AND date = ?2",
    )?;
    let rate: Option<f64> = stmt.query_row(params![currency, date], |row| row.get(0)).optional()?;
    Ok(rate)
}

pub fn save_exchange_rate(conn: &Connection, rate: &ExchangeRate) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO exchange_rates (currency, date, rate) VALUES (?1, ?2, ?3)",
        params![rate.currency, rate.date, rate.rate],
    )?;
    Ok(())
}