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

        }
        _ => {
            println!("Command too long.");
        }
    }
}
