use std::env;
use std::fs;
use std::process::Command;
use std::io::{self, Write};


fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let filename: String = args[0].clone();
    if done(&filename) {
        run(&filename);
    } else {
        println!("please finish your program");
    }
}

fn done(filename: &str) -> bool {
	let file_contents = fs::read_to_string(filename).expect("error in file reading");
    if file_contents.find("I AM NOT DONE") != None {
        false
    } else {
        true
    }
}

fn run(filename: &str) {
    let mut command: String = String::from("java ");
    command.push_str(filename);
    
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", &command]).output().expect("failed to execute process")
    } else {
        Command::new("sh").arg("-c").arg(&command).output().expect("failed to execute process")
    };
    println!("The error file says:");
    io::stderr().write_all(&output.stderr).unwrap();
    println!("The output file says:");
    io::stdout().write_all(&output.stdout).unwrap();
}