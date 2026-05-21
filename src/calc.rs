// use chrono::*;
use rusqlite::Connection;

use super::db::*;
use super::models::*;

// fn make_today_list(mut tree: Vec<HopeBlock>) -> Vec<Task> {
//     eliminate_done(&mut tree);
//     let today: NaiveDate = Local::now().date_naive();
//     let todays_task: Vec<Task> = tree.iter().;
//     for hope_block in tree {
//         let deadline: NaiveDate = hope_block.hope.deadline;
//         let duration = deadline.signed_duration_since(today);
//     }
//     todays_task
// }

pub fn get_single_hope(id: i32, tree: Vec<HopeBlock>) -> Vec<HopeBlock> {
    tree.into_iter().filter(|t| t.hope.id == id).collect()
}

pub fn eliminate_done(tree: &mut Vec<HopeBlock>) {
    for hope_block in tree.into_iter() {
        for process_block in &mut hope_block.process {
            process_block.tasks.retain(|t| !t.is_done);
        }
    }
}

pub fn make_tree(conn: &Connection) -> Vec<HopeBlock> {
    let hopes: Vec<Hope> = match get_hopes(&conn) {
        Ok(hopes) => hopes,
        Err(e) => {
            eprintln!("Error: {}", e);
            Vec::new()
        }
    };
    let processes: Vec<Process> = match get_process(&conn) {
        Ok(process) => process,
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
    let blocks: Vec<HopeBlock> = hopes
        .into_iter()
        .map(|hope| {
            let hope_id = hope.id;
            let related_processes: Vec<Process> = processes
                .iter()
                .filter(|p| p.hope_id == hope_id)
                .cloned()
                .collect();

            let process_block: Vec<ProcessBlock> = related_processes
                .into_iter()
                .map(|process| {
                    let process_id = process.id;
                    let related_task: Vec<Task> = tasks
                        .iter()
                        .filter(|t| t.process_id == Some(process_id))
                        .cloned()
                        .collect();
                    ProcessBlock {
                        process: process,
                        tasks: related_task,
                    }
                })
                .collect();

            HopeBlock {
                hope,
                process: process_block,
            }
        })
        .collect();

    blocks
}
