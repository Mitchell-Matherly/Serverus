use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use toml;



#[derive(Serialize)]
pub struct host_db {
    pub known_hosts: Vec<host>,
}

pub struct host {
    pub name: String,
    pub ip_addr: String,
    pub open_ports: Vec<String>,

}
