mod calc;
mod cli;
mod db;
mod models;

use calc::*;
use clap::Parser;
use cli::*;
use db::*;

fn main() {
    if cfg!(debug_assertions) {
        println!("!!= Now is debug build =!!");
    }
    let data_path = get_db_path();
    let conn = match setup_db(data_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    let args = Cli::parse();
    match args.actions {
        // need to change
        Actions::List { target } => match target {
            Target::Hope { id } => match id {
                Some(id_value) => {
                    let tree = make_tree(&conn);
                    let mut hope_block = get_single_hope(id_value, tree);
                    eliminate_done(&mut hope_block);
                    print_all_task(hope_block);
                }
                None => match get_hopes(&conn) {
                    Ok(hope_vec) => print_hope_list(hope_vec),
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                },
            },
            Target::All => {
                let tree = make_tree(&conn);
                print_all_task(tree);
            }
            _ => {
                let mut tree = make_tree(&conn);
                eliminate_done(&mut tree);
                print_all_task(tree);
            }
        },
        // need to change
        Actions::AddHope { title, deadline } => {
            if let Err(e) = add_hope(&conn, title, deadline) {
                eprintln!("Error: {}", e);
            }
        }
        Actions::AddProcess { title, id } => {
            if let Err(e) = add_process(&conn, title, id) {
                eprintln!("Error: {}", e);
            }
        }
        Actions::AddTask {
            title,
            input,
            action,
            output,
            weight,
            process_id,
        } => {
            if let Err(e) = add_task(&conn, title, input, action, output, weight, process_id) {
                println!("Error: {}", e);
            }
        }
        Actions::Start { id } => {
            println!("!!! Start a task !!! ID: {}", id);
            println!("now happend nothing");
        }
        Actions::Cmp { id } => match complete_task(&conn, id) {
            Ok(c) => {
                println!("Good job !! You Complete ID: {}", id);
                c
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        },
        Actions::Delete { target } => match target {
            Target::Hope { id } => {
                let value = id.expect(" need target's ID --id ");
                if let Err(e) = delete_hope(&conn, value) {
                    eprintln!("Error: {}", e);
                }
            }
            Target::Process { id } => {
                let value = id.expect(" need target's ID --id ");
                if let Err(e) = delete_process(&conn, value) {
                    eprintln!("Error: {}", e);
                }
            }
            Target::Task { id } => {
                let value = id.expect(" need target's ID --id ");
                if let Err(e) = delete_task(&conn, value) {
                    eprintln!("Error: {}", e);
                }
            }
            Target::All => {
                println!("Are you Ok ? you wanna to delete all data??");
                println!("Sorry I cant sport that function")
            }
        },
    }
}
