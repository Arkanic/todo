#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod todo;
mod sql;
mod gui;

use std::{env, fs};
use todo::Todo;

fn main() {
    let mut args:Vec<String> = env::args().collect();
    if args.len() == 1 {
        args.push("list".to_owned()); // default behaviour is to list not done tasks
    }

    let todo_dir = dirs::data_local_dir().unwrap().as_path().join("todo/");

    if !fs::metadata(&todo_dir).is_ok() {
        println!("Database will be stored in {}", todo_dir.as_os_str().to_str().unwrap());
        fs::create_dir(&todo_dir).unwrap();
    }

    let db_path = todo_dir.join("db.sqlite");
    let db_exists = fs::metadata(&db_path).is_ok();
    let connection = sqlite::open(&db_path).unwrap();

    if !db_exists {
        connection.execute(sql::SQL_INIT).unwrap();
    }
    
    let todo = Todo {connection};

    match args[1].as_str() {
        "add" => {
            let mut title = "".to_owned();
            if args.len() >= 3 {
                title = args[2..].join(" ");
            }

            todo.addtask(title);
        }
        "done" | "finish" => {
            if args.len() > 2 {
                let id:i64 = args[2].parse().unwrap();
                todo.changestatus(id, true);
            }
        }
        "redo" => {
            if args.len() > 2 {
                let id:i64 = args[2].parse().unwrap();
                todo.changestatus(id, false);
            }
        }
        "delete" => {
            if args.len() > 2 {
                let id:i64 = args[2].parse().unwrap();
                todo.removetask(id);
            }
        }
        "gui" => {
            let options = eframe::NativeOptions {
                viewport: eframe::egui::ViewportBuilder::default().with_inner_size([400.0, 300.0]),
                ..Default::default()
            };
            eframe::run_native(
                "Todo",
                options,
                Box::new(|_| {
                    Ok(Box::new(gui::TodoApp {
                        todo,
                        textbox_content: "".to_owned(),
                        show_done: false
                    }))
                })
            ).unwrap();
        }
        "list" | "all" | _ => {
            let mut include_done = false;
            if args.len() > 2 && args[2] == "all" || args[1].as_str() == "all" {
                include_done = true;
            }
            for task in todo.gettasks(include_done).iter() {
                println!("{}", task);
            }
        }
    }
}