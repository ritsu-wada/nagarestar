use chrono::*;
use clap::{Parser, Subcommand};

use crate::models::*;

#[derive(Parser)]
#[command(
    name= "ns",
    after_help = format!("Local now: {}", Local::now().date_naive() /* .to_rfc3339() */)
)]
pub struct Cli {
    #[command(subcommand)]
    pub actions: Actions,
}

#[derive(Subcommand)]
pub enum Target {
    #[command(alias = "w")]
    Wish {
        /// your target's id
        #[arg(short, long)]
        id: Option<i32>,
    },
    #[command(alias = "t")]
    Task {
        /// your target's id
        #[arg(short, long)]
        id: Option<i32>,
    },
    #[command(alias = "a")]
    All,
}

#[derive(Subcommand)]
pub enum Actions {
    /// show list of tasks
    #[command(alias = "ls")] // alias for list
    List {
        #[command(subcommand)]
        target: Target,
    },
    /// add wish
    #[command(alias = "aw",after_help = format!("Local now: {}", Local::now().date_naive() /* .to_rfc3339() */))]
    AddWish {
        #[arg(short, long)]
        title: String,
        /// example 1995-08-01 2
        #[arg(short, long)]
        deadline: NaiveDate,
    },
    /// タスクの追加
    #[command(alias = "at")]
    AddTask {
        /// タスクのタイトル
        #[arg(short, long)]
        title: String,
        /// 準備、必要なもの場所
        #[arg(short, long)]
        input: String,
        /// 何をする作業？
        #[arg(short, long)]
        action: String,
        /// 何がゴール？
        #[arg(short, long)]
        output: String,
        /// 1: 確実に1時間で終わる 2: 1時間で終わるだろうが不安 3: 未知の作業
        #[arg(short, long, default_value_t = 1)]
        weight: i32,
        /// related wish's ID
        #[arg(short, long)]
        root_id: i32,
    },
    Start {
        /// your target task's ID
        #[arg(short, long)]
        id: i32,
    },
    /// change to state complete
    Cmp {
        /// your target task's ID
        #[arg(short, long)]
        id: i32,
    },
    /// delete data
    #[command(alias = "d")]
    Delete {
        #[command(subcommand)]
        target: Target,
    },
}

pub fn print_wishs(wish_vec: Vec<Wish>) {
    for wish in wish_vec {
        println!("[wish ID:{}]:", wish.id);
        println!(" DeadLine: {}", wish.deadline);
        println!(" TITLE: {}", wish.title);
    }
}

pub fn print_all_task(tree: Vec<WishBlock>) {
    let print_related_tasks = |task: &Task| {
        println!("　　├─[Task] ID: {} -", task.id);
        println!("　　│  Title: {}", task.title);
        println!("　　│  Input: {}", task.input);
        println!("　　│  Action: {}", task.action);
        println!("　　│  Output: {}", task.output);
        println!("　　└  Weight: {}", task.weight);
    };
    let print_wish_block = |block: &WishBlock| {
        println!("[wish ID:{}]:", block.wish.id);
        println!(" DeadLine: {}", block.wish.deadline);
        println!(" TITLE: {}", block.wish.title);
    };
    for block in tree {
        print_wish_block(&block);
        for task in block.tasks {
            print_related_tasks(&task);
        }
    }
}
