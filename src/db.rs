use chrono::prelude::*;
use rusqlite::{Connection, Result};
use std::path::PathBuf;

use super::models::*;

pub fn get_db_path() -> PathBuf {
    #[cfg(not(debug_assertions))]
    {
        use {directories::ProjectDirs, std::fs};

        let proj_dirs = ProjectDirs::from("ns_com", "ns_org", "nagarestar").expect("Path failed");
        let data_dir = proj_dirs.data_dir();
        fs::create_dir_all(data_dir).expect("faild to create dirctories");
        data_dir.join("data.db")
    }
    #[cfg(debug_assertions)]
    {
        PathBuf::from("test.db")
    }
}

pub fn setup_db(data_path: PathBuf) -> Result<Connection> {
    let conn = Connection::open(data_path)?;
    conn.execute("PRAGMA foreign_keys = ON;", [])?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS wishes (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            deadline DATETIME NOT NULL
        )",
        (),
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            input TEXT NOT NULL,
            action TEXT NOT NULL,
            output TEXT NOT NULL,
            weight INTEGER NOT NULL, 
            root_id INTEGER NOT NULL,
            is_done BOOLEAN NOT NULL DEFAULT 0,
            FOREIGN KEY (root_id) REFERENCES wishes(id) ON DELETE CASCADE
        )",
        (),
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS routines (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            time DATETIME NOT NULL,
            weight INTEGER NOT NULL,
            root_id INTEGER,
            FOREIGN KEY (root_id) REFERENCES wishes(id)
        )",
        (),
    )?;

    Ok(conn)
}

pub fn add_wish(conn: &Connection, title: String, deadline: NaiveDate) -> Result<()> {
    conn.execute(
        "INSERT INTO wishes (title, deadline) VALUES (?1, ?2)",
        (title, deadline),
    )?;
    Ok(())
}

pub fn add_task(
    conn: &Connection,
    title: String,
    input: String,
    action: String,
    output: String,
    weight: i32,
    root_id: i32,
) -> Result<()> {
    // 静的ステークホルダー、配列化タプルを渡すことができる
    conn.execute(
        "INSERT INTO tasks (title, input, action, output, weight, root_id) VALUES (?1,?2,?3,?4,?5,?6)",
        (title, input, action, output, weight, root_id),
    )?;
    Ok(())
}

pub fn get_tasks(conn: &Connection) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, input, action, output, weight, root_id, is_done FROM tasks ORDER BY is_done ASC",
    )?;
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            input: row.get(2)?,
            action: row.get(3)?,
            output: row.get(4)?,
            weight: row.get(5)?,
            root_id: row.get(6)?,
            is_done: row.get(7)?,
        })
    })?;
    let tasks: Result<Vec<Task>> = task_iter.collect();
    tasks
}

pub fn get_wishes(conn: &Connection) -> Result<Vec<Wish>> {
    let mut stmt = conn.prepare("SELECT id, title, deadline FROM wishes ORDER BY deadline ASC")?;
    let wish_iter = stmt.query_map([], |row| {
        Ok(Wish {
            id: row.get(0)?,
            title: row.get(1)?,
            deadline: row.get(2)?,
        })
    })?;
    let wishes: Result<Vec<Wish>> = wish_iter.collect();
    wishes
}

pub fn complete_task(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("UPDATE tasks SET is_done = 1 WHERE id = (?1)", (id,))?;
    Ok(())
}

pub fn delete_task(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM tasks WHERE id = (?1)", (id,))?;
    Ok(())
}
pub fn delete_wish(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM wishes WHERE id = (?1)", (id,))?;
    Ok(())
}
