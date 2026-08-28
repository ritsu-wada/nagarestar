use rusqlite::{Connection, Result};
use std::path::PathBuf;

use super::models::*;

// // 外部から呼び出せるように
// #[macro_export]

// macro_rules! {
//     (
//         table: $table_name:expr,
//         struct: $struct_name:ident,
//         id: $id_field:ident,
//         fields: {
//             $( $field:ident : $ftype:ty => $sql_type:expr ),* $(,)?
//         }
//     ) => {
//         impl $struct_name {
//             ///テーブルを作成するところ
//             pub fn create_table(conn: &rusqlite::Connection) -> rustqlite::Result<()> {
//                 let mut sql = format!(
//                     "CREATE TABLE IF NOT EXISTS {} ({} INTEGER PRIMARY KEY AUTOINCREMENT",
//                     $table_name,
//                 stringfy!($id_field)
//                 );
//                 $(
//                     sql.push_str(&format!(", {} {}", stringify!($field), $sql_type));
//                 )*
//                 sql.push(')');
//             conn.execute(&sql,[])?;
//             Ok(())
//             }

//             /// rusqlite::Rowから構造体を復元するらしい
//             pub fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
//                 Ok(Self {
//                     $id_field: row.get(stringfy!($id_field))?,
//                     $(
//                         $field: row.get(stringify!($field))?,
//                     )*
//                 })
//             }

//             /// 全件取得するらしい
//             pub fn find_all(conn: &rusqlite::Connection) -> rusqlite::Result<Vec<Self>> {
//                 let sql = fromat!(
//                     "SELECT {},{} FROM {}",
//                     stringify!($id_field),
//                     let mut stmt = conn.prepare(&sql)?;
//                     let rows = stmt.query_map([], |row| Self::from_row(row))?;
//                     let mut results = Vec::new();
//                     for item in rows {
//                         results.push(item?);
//                     }
//                     Ok(results)
//                 )
//             }
//         }
//     };
// }

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
            deadline DATETIME NOT NULL,
            priority INTEGER NOT NULL
        )",
        (),
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            root_id INTEGER NOT NULL,
            title TEXT NOT NULL,

            input TEXT NOT NULL,
            action TEXT NOT NULL,
            output TEXT NOT NULL,

            not_to_do TEXT,
            scheduled_at TEXT,
            weight INTEGER NOT NULL, 

            FOREIGN KEY (root_id) REFERENCES wishes(id) ON DELETE CASCADE
        )",
        (),
    )?;

    conn.execute(
        "
            CREATE TABLE IF NOT EXISTS done_tasks {
                id INTEGER PRIMARY KEY,
                root_id INTEGER NOT NULL,
                title TEXT NOT NULL,
                completed_at DATETIME NOT NULL,
            }
        ",
        (),
    )?;

    Ok(conn)
}

pub fn edit_wish(conn: &Connection, wish: Wish) -> Result<()> {
    conn.execute(
        "UPDATE wish
         SET title = ?1,
             deadline = ?2,
             priority = ?3
         WHERE id = ?4",
        (wish.title, wish.deadline, wish.priority, wish.id),
    )?;
    Ok(())
}

pub fn edit_task(conn: &Connection, task: Task) -> Result<()> {
    // 静的ステークホルダー、配列化タプルを渡すことができる
    conn.execute(
        "UPDATE tasks
         SET root_id = ?1, title = ?2, input = ?3, action = ?4,
          output = ?5, not_to_do = ?6, scheduled_at = ?7, weight = ?8, 
         WHERE id = ?9",
        (
            task.root_id,
            task.title,
            task.input,
            task.action,
            task.output,
            task.not_to_do,
            task.scheduled_at,
            task.weight,
            task.id,
        ),
    )?;
    Ok(())
}

pub fn add_wish(conn: &Connection, wish: Wish) -> Result<()> {
    conn.execute(
        "INSERT INTO wishes (title, deadline, priority) VALUES (?1, ?2, ?3)",
        (wish.title, wish.deadline, wish.priority),
    )?;
    Ok(())
}

pub fn add_task(conn: &Connection, task: Task) -> Result<()> {
    // 静的ステークホルダー、配列化タプルを渡すことができる
    conn.execute(
        "INSERT INTO tasks (root_id, title, input, action, output, not_to_do, scheduled_at, weight) VALUES (?1,?2,?3,?4,?5,?6,?7,?8)",
        (task.root_id,task.title,task.input,task.action,task.output,task.not_to_do,task.scheduled_at,task.weight),
    )?;
    Ok(())
}

pub fn get_tasks(conn: &Connection) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare(
        "SELECT id, root_id, title, input, action, output, not_to_do, scheduled_at, weight FROM tasks",
    )?;
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            root_id: row.get(1)?,
            title: row.get(2)?,

            input: row.get(3)?,
            action: row.get(4)?,
            output: row.get(5)?,

            not_to_do: row.get(6)?,
            scheduled_at: row.get(7)?,
            weight: row.get(8)?,
        })
    })?;
    let tasks: Result<Vec<Task>> = task_iter.collect();
    tasks
}

pub fn get_wishes(conn: &Connection) -> Result<Vec<Wish>> {
    let mut stmt =
        conn.prepare("SELECT id, title, deadline, priority FROM wishes ORDER BY deadline ASC")?;
    let wish_iter = stmt.query_map([], |row| {
        Ok(Wish {
            id: row.get(0)?,
            title: row.get(1)?,
            deadline: row.get(2)?,
            priority: row.get(3)?,
        })
    })?;
    let wishes: Result<Vec<Wish>> = wish_iter.collect();
    wishes
}

pub fn delete_task(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM tasks WHERE id = (?1)", (id,))?;
    Ok(())
}
//before
pub fn delete_wish(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM wishes WHERE id = (?1)", (id,))?;
    Ok(())
}
