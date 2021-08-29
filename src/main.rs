use std::env;
use std::fs;
use std::process::Command;
use std::io::{self, Write};


fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let files_vector: Vec<String> = vec![String::from("Into"),
                                        String::from("HelloWorld"),
                                        String::from("Comments"),
                                        String::from("Variables"),
                                        String::from("Input"),
                                        String::from("PrimitiveOperators"),
                                        String::from("IncrementDecrement"),
                                        String::from("Strings"),
                                        String::from("Conditionals"),
                                        String::from("NestedConditionals"),
                                        String::from("LogicalStatements"),
                                        String::from("Booleans"),
                                        String::from("SwitchStatement"),
                                        String::from("WhileLoop"),
                                        String::from("ForLoop"),
                                        String::from("DoWhileLoop"),
                                        String::from("NestedLoops"),
                                        String::from("Arrays"),
                                        String::from("SummingElements"),
                                        String::from("Foreach"),
                                        String::from("MultidimensionalArrays"),
                                        String::from("SortingAlgorithm"),
                                        String::from("SearchingAlgorithm"),
                                        String::from("Classes"),
                                        String::from("Members"),
                                        String::from("Methods"),
                                        String::from("ClassesFromZero"),
                                        String::from("HandsOnObjects"),
                                        String::from("AccessModifiers"),
                                        String::from("GettersSetters"),
                                        String::from("Final"),
                                        String::from("Imports"),
                                        String::from("Inheritance"),
                                        String::from("Polymorphism"),
                                        String::from("FunctionOverloading"),
                                        String::from("FunctionOverriding"),
                                        String::from("Enums"),
                                        String::from("Abstract"),
                                        String::from("Interface")];
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



