#![allow(unused)]
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut fil = File::open("output.txt").expect("unable to open file");
    let mut lines = io::BufReader::new(fil).lines();
    let mut cont = String::new();

    let mut scalls = ["execve", "openat", "dup2", "unlinkat", "mkdir", "chdir"];

    for line in lines {
        match line {
            Ok(s) => {
                let splitted: Vec<&str> = s.split(" ").collect();
                let splitted2: Vec<&str> = s.split("\"").collect();

                out_gen(&splitted, &mut String::new(), &splitted2, &scalls)
            }
            _ => continue,
        }
    }
}

fn out_gen(vtr: &Vec<&str>, otp: &mut String, argm: &Vec<&str>, scalls: &[&str]) {
    match vtr[1].find(scalls[0]) {
        Some(j) => {
            if argm[1] == "/usr/bin/ln" {
                println!("Created hardlink from {} to {}", argm[5], argm[7]);
            }
            if argm[1] == "/usr/bin/mkdir" {
                println!("Created directory {}", argm[7]);
                return;
            }
            if argm[1] == "/usr/bin/cat" {
                println!("writing to file {} through cat", argm[5]);
                return;
            }
            println!("Called execve with argument {}", argm[1]);
            return;
        }
        _ => (),
    };

    match vtr[1].find(scalls[1]) {
        Some(j) => {
            if let Some(x) = argm[1].find("/usr/lib/") {
                return;
            } else if let Some(x) = argm[1].find("/etc/") {
                return;
            }
            println!("opened file {}", argm[1]);
            return;
        }
        _ => (),
    };

    //    match vtr[1].find(scalls[2]) {
    //        Some(j) => {
    //            println!("duplicated file descripitor {}{}", vtr[1], vtr[2]);
    //            return;
    //        }
    //        _ =>  ()
    //    };

    match vtr[1].find(scalls[3]) {
        Some(j) => {
            println!("deleted file {}", argm[1]);
            return;
        }
        _ => (),
    };

    match vtr[1].find(scalls[5]) {
        Some(j) => {
            println!("change directory to {}", argm[1]);
            return;
        }
        _ => (),
    };

    match vtr[1].find(scalls[4]) {
        Some(j) => {
            if argm[1] == "/usr/bin/mkdir" {
                return;
            }
            println!("make directory {}", argm[1]);
            return;
        }
        _ => (),
    };
}
