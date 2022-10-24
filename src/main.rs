

#![allow(unused)]
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Parser)]
#[grammar= "strace.pest"]
pub struct Syscallpar;


fn main() {
    println!("Hello, world!");

    let mut fil = File::open("output.txt").expect("unable to open file");
    //let mut fil = read_lines("output.txt").expect("unable to open file");
    let mut lines = io::BufReader::new(fil).lines();
    let mut cont = String::new();
   // fil.read_to_string(&mut cont).expect("Failed");
    
    let mut binding = lines.next().unwrap().unwrap();
    let mut scalls = ["execve", "openat", "dup2", "unlinkat", "mkdir", "chdir"];
    
//    let successful_parse = Syscallpar::parse(Rule::syscall, "517604 execve(\"./tut.sh\", [\"./tut.sh\"], 0x7ffd69d77700 /* 44 vars */) = 0");
    let successful_parse = Syscallpar::parse(Rule::syscall, &binding);
//    let successful_parse = Syscallpar::parse(Rule::args, &binding);
    let splitted: Vec<&str> = binding.split(" ").collect();
    let splitted2: Vec<&str> = binding.split("\"").collect();
    println!("{:?}", successful_parse);
    println!("{:?}", splitted);
    println!("{:?}", splitted2);
    out_gen(&splitted, &mut String::new(), &scalls);
}


fn out_gen(vtr: &Vec<&str>, otp: &mut String, scalls: &[str]) {
    println!("{:?}", vtr[1].find("execve"));

}
