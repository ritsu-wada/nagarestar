mod calc;
mod cli;
mod db;
mod models;

use calc::*;
use clap::Parser;
use cli::*;
use db::*;
use inquire::Text;

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
            Some(Target::Wish { id }) => match id {
                Some(id) => {
                    let tree = make_tree(&conn);
                    let wish_block = get_single_wish(id, tree);
                    print_all_task(wish_block);
                }
                _ => match get_wishes(&conn) {
                    Ok(wish_vec) => print_wishes(wish_vec),
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                },
            },
            Some(Target::All) => {
                let tree = make_tree(&conn);
                print_all_task(tree);
            }
            _ => {
                let mut tree = make_tree(&conn);
                eliminate_done(&mut tree);
                let next_task = calc_next_todo(tree);
                if let Some(task) = next_task {
                    print_task(task);
                }
            }
        },
        // need to change
        Actions::AddWish { title, deadline } => {
            if let Err(e) = add_wish(&conn, title, deadline) {
                eprintln!("Error: {}", e);
            }
        }
        Actions::AddTask {
            title,
            input,
            action,
            output,
            weight,
            root_id,
        } => {
            if let Err(e) = add_task(&conn, title, input, action, output, weight, root_id) {
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
        Actions::Edit {
            root_id,
            title,
            input,
            action,
            output,
            weight,
        } => {
            if let Err(e) = edit_task(&conn, root_id, title, input, action, output, weight) {
                println!("Error: {}", e);
            }
        }
        Actions::Delete { target } => match target {
            Target::Wish { id } => {
                let value = id.expect(" need target's ID --id ");
                if let Err(e) = delete_wish(&conn, value) {
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
        Actions::Ctl => {
            let test = Text::new("This is a test context")
                .with_default("Ritsu")
                .with_help_message("Please type some message")
                .prompt();
            match test {
                Ok(context) => println!("this is your context {}", context),
                Err(_) => println!("Error"),
            }
        }
    }
}
