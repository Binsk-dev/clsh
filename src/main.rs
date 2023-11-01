use std::io::prelude::*;
use std::path::Path;
use std::{env, fs::File};

fn main() {
    let args: Vec<String> = env::args().collect();
    let hosts = extract_hosts(args);
    println!("{:?}", hosts);
}

fn extract_hosts(args: Vec<String>) -> Vec<String> {
    let hosts: Vec<String> = match args[1].as_str() {
        "-h" => get_hosts_from_string(args[2].as_str()),
        "--hostfile" => {
            let mut buf = String::new();
            let path = Path::new(args[2].as_str());
            
            let _ = match File::open(path) {
                Ok(mut file) => file.read_to_string(&mut buf),
                Err(_) => panic!("Fail to find hostfile"),
            };

            let mut hosts: Vec<String> = Vec::new();
            for ln in buf.split("\n") {
                for (_, item) in ln.split_whitespace().enumerate().filter(|x| x.0 % 2 == 1) {
                    hosts.push(item.to_string());
                }
            }
            hosts
        }
        _ => {
            match env::var_os("CLSH_HOSTS") {
                Some(hosts) => {
                    if let Some(something) = hosts.to_str() {
                        get_hosts_from_string(something)
                    } else {
                        panic!("Can't convert to string...")
                    }
                },
                None => panic!("Can't find CLSH_HOST ENV..."),
            } 
        }
    };
    hosts
}

fn get_hosts_from_string(host_string: &str) -> Vec<String> {
    host_string.split(",").map(|x| x.to_string()).collect()
}