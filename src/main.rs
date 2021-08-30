use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{self,Command};

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();
    if args_len < 2 {
        // No command
        eprintln!("There is no command identified. Use \"javalings help\" for help");
        process::exit(1);
    }
    let command = &args[1];
    let mut filename: String;
    
    if command == "list" {
        if args_len > 2 {
            // Unrecognized argument
            eprintln!("Unrecognized argument: {}", &args[2]);
            process::exit(1);
        }
        list();
    } else if command == "run" {
        if args.len()< 3 {
            // No file for the line command
            eprintln!("There is no file identified. Use \"javalings help\" for help.");
            process::exit(1);
        } else if args.len()> 3 {
            // Unrecognized argument
            eprintln!("Unrecognized argument: {}", &args[3]);
            process::exit(1);
        }
        filename = args[2].clone();
        filename = check_file(filename);
        run(&filename);
    } else if command == "verify" {
        if args_len > 2 {
            // Unrecognized argument
            eprintln!("Unrecognized argument: {}", &args[2]);
            process::exit(1);
        }
        verify();
    } else if command == "watch" {
        if args_len > 2 {
            // Unrecognized argument
            eprintln!("Unrecognized argument: {}", &args[2]);
            process::exit(1);
        }
        for _i in 0..50 {
            println!();
        }
        verify();
    } else {
        // Wrong command
        eprintln!("The  command identified. Use \"javalings help\" for help");
        process::exit(1);
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
        Command::new("cmd")
            .args(&["/C", &command])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .expect("failed to execute process")
    };
    if !output.status.success(){
        println!("The error file says:\n");
        io::stderr().write_all(&output.stderr).unwrap();
        println!("The output file says:\n");
        io::stdout().write_all(&output.stdout).unwrap();
    } else {
        println!("The output file says:");
        io::stdout().write_all(&output.stdout).unwrap();
    }
}

fn match_name_to_path(filename: &Path, name: &str) -> Option<String> {
    for entry in fs::read_dir(filename).unwrap() {
        let entry = entry.unwrap();
        if entry.path().is_file() {
            if entry.file_name().into_string().unwrap().contains(&name) {
                return Some(entry.file_name().into_string().unwrap());
            }
        } else {
            let r = match_name_to_path(&entry.path(), &name);
            if r != None {
                if cfg!(target_os = "windows") {
                    return Some((entry.path().to_str().unwrap().to_string() + "\\") + &r.unwrap());
                } else {
                    return Some((entry.path().to_str().unwrap().to_string() + "/") + &r.unwrap());
                }
            }
        }
    }
    None
}

fn get_files() -> Vec<String> {
    vec![
        String::from("Intro"),
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
        String::from("Interface"),
    ]
}
fn list() {
    let mut counter: f64= 0.0;

    for file in get_files() {
        let path: String = match_name_to_path(Path::new("."), &file).unwrap()[12..].to_owned();
        let finished = done(&path);
        let len = file.len();
        print!("{}", file);
        for _i in 0..3 - len / 8 {
            print!("\t");
        }
        print!("{}", path);
        for _i in 0..3 - len / 8 {
            print!("\t");
        }
        if len % 8 == 0 {
            print!("\t");
        }
        if file == "Interface" || file == "Variables" {
            print!("\t");
        }
        if finished {
            counter += 1.0;
            println!("Done");
        } else {
            println!("Pending");
        }
    }
    println!("\nProgress: You completed {} / {} exercises ({} %).", counter as u8, get_files().len(), counter * 100.0 / (get_files().len() as f64));
}

fn check_file(mut filename: String) -> String{
    if !filename.ends_with(".java") {
        let temp = match_name_to_path(Path::new("."), &filename);
        if temp != None {
            filename = temp.unwrap()[12..].to_owned();
        } else {
            // The file doesn't exist
            eprintln!("The file you entered as an argument is not in the exercises");
            process::exit(1);
        }
    }
    if !Path::new(&filename).is_file() {
        // Unrecognized file
        eprintln!("{} is not a file in this directory, use it's name to check/run it or use \"javalings help\" for help.",filename);
        process::exit(1);
    }
    filename
}

fn verify() {
    let files = get_files();
    for file in files {
        let filename: String = match_name_to_path(Path::new("."), &file).unwrap()[12..].to_owned();
        let mut command: String = String::from("java ");
        command.push_str(&filename);
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", &command])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(&command)
                .output()
                .expect("failed to execute process")
        };
        if output.status.success() {
            println!("Successfully ran {}", filename);
        } else {
            println!("Compiling of {} failed\n",filename);
            println!("The error file says:\n");
            io::stderr().write_all(&output.stderr).unwrap();
            println!("The output file says:\n");
            io::stdout().write_all(&output.stdout).unwrap();
            process::exit(1);    
        }
    }
}
