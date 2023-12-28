use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Record {
    pub key: String,
    pub value: String,
}

impl Record {
    pub fn new(key: String, value: String) -> Self {
        Record { key, value }
    }
}

// Write a record to a file
pub fn write_record(record: &Record, filename: &str) -> io::Result<()> {
    let mut records = match read_records(filename) {
        Ok(existing_records) => existing_records,
        Err(_) => Vec::new(),
    };

    records.push(record.clone());

    let file = File::create(filename)?;
    serde_json::to_writer(file, &record)?;
    Ok(())
}

// Read all records from a file
pub fn read_records(filename: &str) -> io::Result<Vec<Record>> {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                return Ok(Vec::new());
            } else {
                return Err(e);
            }
        }
    };
    let reader = BufReader::new(file);

    match serde_json::from_reader(reader) {
        Ok(data) => Ok(data),
        Err(e) => {
            eprintln!("Failed to parse records: {}", e);
            Ok(Vec::new())
        }
    }
}
