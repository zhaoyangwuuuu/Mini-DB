use mini_db::{read_records, write_record, Record};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprint!("Usage: mini_db <command> [arguments]");
        process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "insert" => insert_record(&args),
        "get" => get_records(),
        _ => {
            eprintln!("Invalid command");
            process::exit(1);
        }
    }
}

fn insert_record(args: &[String]) {
    if args.len() < 4 {
        eprint!("Usage: mini_db insert <key> <value>");
        process::exit(1);
    }

    let key = &args[2];
    let value = &args[3];
    let record = Record::new(key.clone(), value.clone());

    if let Err(e) = write_record(&record, "db.txt") {
        eprint!("Error writing record: {}", e);
        process::exit(1);
    }
}

fn get_records() {
    match read_records("db.txt") {
        Ok(records) => {
            for record in records {
                println!("{}: {}", record.key, record.value);
            }
        }
        Err(e) => {
            eprintln!("Error reading records: {}", e);
            process::exit(1);
        }
    }
}
