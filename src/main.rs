use std::io::prelude::*;
use std::path::Path;
use std::{env, fs::File};

fn main() {
    let args: Vec<String> = env::args().collect();

    let hosts: Vec<String> = match args[1].as_str() {
        "-h" => args[2].split(",").map(|x| x.to_string()).collect(),
        "--hostfile" => {
            let path = Path::new("clusterfile");

            let mut file = match File::open(&path) {
                Ok(file) => file,
                Err(why) => panic!("Fail to open: {}, {}", path.display(), why),
            };

            let mut buf = String::new();
            match file.read_to_string(&mut buf) {
                Ok(_) => (),
                Err(why) => panic!("Fail to read hostfile. {}", why),
            }

            let mut hosts: Vec<String> = Vec::new();
            for ln in buf.split("\n") {
                for (_, item) in ln.split_whitespace().enumerate().filter(|x| x.0 % 2 == 1) {
                    hosts.push(item.to_string());
                }
            }
            hosts
        }
        _ => panic!("Can't find to host"),
    };
    println!("{:?}", hosts);
}