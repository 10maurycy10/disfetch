use std::cmp::max;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use serde_derive::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub username: String,
    pub hostname: String,
    pub logo: String,
    lines: Vec<(String, String)>
}

fn main() {
    let config: Config = serde_yaml::from_reader(BufReader::new(File::open("config.yaml").expect("Cant open config file!"))).expect("Invalid config");
    
    let mut lines: &Vec<(String, String)> = &config.lines;
    
    let logo: Vec<_> = BufReader::new(File::open(config.logo).expect("Failed to open logo")).lines().map(|x| x.unwrap()).collect();
    
    let hight = max(logo.len(), lines.len() + 4);
    let mut disfetch: Vec<String> = Vec::new();
    for i in 0..hight {
            disfetch.push("".to_owned())
    }
    let blank = hight as isize - logo.len() as isize;
    if blank > 0 {
        for i in 0..blank {
            disfetch[logo.len() + i as usize].push_str(&logo[logo.len() - 1])
        }
    }
    
    
    for i in 0..logo.len() {
        disfetch[i].push_str(&logo[i]);
    }
    
    disfetch[0].push_str(&format!("\x1b[36;1m{}\x1b[0m@\x1b[36;1m{}", config.username, config.hostname));
    disfetch[1].push_str("\x1b[0m--------");
    
    for i in 0..lines.len() {
        disfetch[i+2].push_str(&format!("{}: \x1b[0m{}",lines[i].0, lines[i].1));
    }
    
    disfetch[hight - 2].push_str("\x1b[1;30m███\x1b[1;31m███\x1b[1;32m███\x1b[1;33m███\x1b[1;34m███\x1b[1;35m███\x1b[1;36m███\x1b[1;37m███\x1b[0m");
    disfetch[hight - 1].push_str("\x1b[2;30m███\x1b[2;31m███\x1b[2;32m███\x1b[2;33m███\x1b[2;34m███\x1b[2;35m███\x1b[2;36m███\x1b[2;37m███\x1b[0m");
    
    println!("```ansi");
    for line in disfetch {
        println!("{}", &line);
    }
    println!("```");
}
