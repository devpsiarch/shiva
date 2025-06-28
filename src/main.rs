mod job;
use crate::job::*;
use std::env;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use serde_json::{Result, Value,json,to_writer_pretty};

fn main() {
    let args: Vec<String> = env::args().collect();     
    if args.len() < 2 {
        println!("Usage: <cmd> <args> ...");
        return;
    }
    match args.len() {
        2 => {
            match args[1].as_str() {
                "help" => {
                    // call help printer
                }
                "list" => {
                    // call list_jobs
                    Job::list_services();
                }
                "create" => {
                    // call wizard_promt job creator for now
                    Job::create_service();
                }
                _ => {
                    println!("Unreconized command  \"{}\".",args[1]);
                }
            }
        }
        3 => {
            match args.get(1).map(|s| s.as_str()) {
                Some("enable") | Some("disable") | Some("kill") | Some("start") | Some("stop") | Some("status") => {
                    let cmd = args.get(1).expect("error getting the cmd").as_str();
                    if let Some(service) = args.get(2) {
                        Job::alter_service(cmd,service);
                    } else {
                        println!("Missing operant after \"{}\".",cmd);
                    }
                }
                Some("remove") => {
                    let cmd = args.get(1).expect("error getting the cmd").as_str();
                    if let Some(service) = args.get(2) {
                        Job::remove_service(service);
                    } else {
                        println!("Missing operant after \"{}\".",cmd);
                    }
                }
                Some("log") => {
                    let cmd = args.get(1).expect("error getting the cmd").as_str();
                    if let Some(service) = args.get(2) {
                        Job::log_service(service);
                    } else {
                        println!("Missing operant after \"{}\".",cmd);
                    }
                }
                Some("backup") => {
                    let cmd = args.get(1).expect("error getting the cmd").as_str();
                    if let Some(service) = args.get(2) {
                        Job::backup_service(service);
                    } else {
                        println!("Missing operant after \"{}\".",cmd);
                    }
                }
                Some(other) => {
                    println!("Unreconized command  \"{}\".",args[1]);
                }
                None => {
                    println!("Nothing found");
                }
            }
        }
        _ => {
            println!("Command too long.");
        }
    }
    
}
