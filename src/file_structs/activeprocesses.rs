use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use toml;



#[derive(Serialize)]
pub struct active_processes {
    pub processes: Vec<Process>,
}

pub struct Process {
    pub name: String,
    pub status: String,
    pub current_provider: (String, String),
    pub list_of_hosts: Vec<String>,
}

