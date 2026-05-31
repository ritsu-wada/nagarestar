// use chrono::*;
use rusqlite::Connection;

use super::db::*;
use super::models::*;

pub fn get_single_wish(id: i32, tree: Vec<WishBlock>) -> Vec<WishBlock> {
    tree.into_iter().filter(|t| t.wish.id == id).collect()
}

pub fn eliminate_done(tree: &mut Vec<WishBlock>) {
    for wish_block in tree.into_iter() {
        wish_block.tasks.retain(|t| !t.is_done);
    }
}

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
