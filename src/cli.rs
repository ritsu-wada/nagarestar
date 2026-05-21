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
    #[command(alias = "h")]
    Hope {
        /// your target's id
        #[arg(short, long)]
        id: Option<i32>,
    },
    #[command(alias = "p")]
    Process {
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
    /// add hope
    #[command(alias = "ah",after_help = format!("Local now: {}", Local::now().date_naive() /* .to_rfc3339() */))]
    AddHope {
        #[arg(short, long)]
        title: String,
        /// example 1995-08-01 2
        #[arg(short, long)]
        deadline: NaiveDate,
    },
    /// add process
    #[command(alias = "ap")]
    AddProcess {
        /// the process's title
        #[arg(short, long)]
        title: String,
        /// related hope's ID
        #[arg(short, long)]
        id: i32,
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
        /// related Process's ID
        #[arg(short, long)]
        process_id: i32,
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

pub fn print_hope_list(hope_vec: Vec<Hope>) {
    for hope in hope_vec {
        println!("[Hope ID:{}]:", hope.id);
        println!(" DeadLine: {}", hope.deadline);
        println!(" TITLE: {}", hope.title);
    }
}

// pub fn print_one_hope(hope_block: HopeBlock) {
//     let hope = hope_block.hope;
//     println!("[Hope ID: {} Deadline: {}]", hope.id, hope.deadline);
//     println!(" Title: {}", hope.title);

// }

pub fn print_all_task(tree: Vec<HopeBlock>) {
    println!("=== Task Tree ===");
    let print_related_tasks = |task: &Task| {
        println!("　　├─[Task] ID: {} -", task.id);
        println!("　　│  Title: {}", task.title);
        println!("　　│  Input: {}", task.input);
        println!("　　│  Action: {}", task.action);
        println!("　　│  Output: {}", task.output);
        println!("　　└  Weight: {}", task.weight);
    };
    let print_hope_block = |block: &HopeBlock| {
        println!("[Hope ID:{}]:", block.hope.id);
        println!(" DeadLine: {}", block.hope.deadline);
        println!(" TITLE: {}", block.hope.title);
    };
    let print_process = |process: &Process| {
        println!("　├─[Process] ID: {}", process.id);
        println!("　└  Title: {}", process.title);
    };
    for block in tree {
        print_hope_block(&block);
        for process_block in block.process {
            print_process(&process_block.process);
            for task in process_block.tasks {
                print_related_tasks(&task);
            }
        }
    }
}
