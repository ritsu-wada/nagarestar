use chrono::prelude::*;

#[derive(Clone)]
pub struct Wish {
    pub id: i32,
    pub title: String,
    pub deadline: NaiveDate,
}

#[derive(Clone)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub input: String,
    pub action: String,
    pub output: String,
    pub weight: i32,
    pub root_id: i32,
    pub is_done: bool,
}

#[derive(Clone)]
pub struct WishBlock {
    pub wish: Wish,
    pub tasks: Vec<Task>,
}
