// use chrono::*;
use rusqlite::Connection;

use super::db::*;
use super::models::*;

// 現時点ではdbから期限が近い物順にデータが来ること
// を前提としているため日付を見るようにする必要がある
pub fn calc_next_todo(tree: Vec<WishBlock>) -> Option<Task> {
    let task: Option<Task> = match tree.first() {
        Some(wish_block) => wish_block.tasks.first().cloned(),
        None => None,
    };
    task
}

pub fn get_single_wish(id: i32, tree: Vec<WishBlock>) -> Vec<WishBlock> {
    tree.into_iter().filter(|t| t.wish.id == id).collect()
}

// make data include all of data
pub fn make_tree(conn: &Connection) -> Vec<WishBlock> {
    let wishes: Vec<Wish> = match get_wishes(&conn) {
        Ok(wishes) => wishes,
        Err(e) => {
            eprintln!("Error: {}", e);
            Vec::new()
        }
    };
    let tasks: Vec<Task> = match get_tasks(&conn) {
        Ok(tasks) => tasks,
        Err(e) => {
            eprintln!("Error: {}", e);
            Vec::new()
        }
    };
    let blocks: Vec<WishBlock> = wishes
        .into_iter()
        .map(|wish| {
            let related_tasks = tasks
                .iter()
                .filter(|task| task.root_id == wish.id)
                .cloned()
                .collect();
            WishBlock {
                wish: wish,
                tasks: related_tasks,
            }
        })
        .collect();
    blocks
}
