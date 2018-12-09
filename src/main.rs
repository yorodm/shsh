extern crate rustyline;
use std::env;
use std::io;
use std::process::Command;

fn change_directory(args: &[&str]) -> io::Result<()> {
    let path = args.join(" ");
    env::set_current_dir(path)
}

fn print_working_directory(args: &[&str]) -> io::Result<()> {
    match env::current_dir() {
        Ok(cwd) => {
            println!("Current working dir: {}", cwd.as_path().display());
            Ok(())
        }
        Err(err) => Err(err),
    }
}

fn echo_input(args: &[&str]) -> io::Result<()> {
    println!("{}", args.join(" "));
    return Ok(());
}

fn execute_external_commnd(args: &[&str]) -> io::Result<()> {
    match Command::new(args[0]).args(args[1..].iter()).spawn() {
        Ok(mut child) => match child.wait() {
            Ok(stat) => if stat.success() {
                Ok(())
            } else {
                Err(io::Error::new(io::ErrorKind::Other, "Command failed!"))
            },
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

fn parse_and_execute(line: &str) {
    let v: Vec<&str> = line.split(' ').collect();
    match v[0] {
        "cd" => match change_directory(&v[1..]) {
            Err(err) => println!("{:?}", err),
            _ => (),
        },
        "pwd" => match print_working_directory(&v[1..]) {
            Err(err) => println!("{:?}", err),
            _ => (),
        },
        "echo" => match echo_input(&v[1..]) {
            _ => (),
        },
        _ => match execute_external_commnd(&v) {
            Err(err) => println!("{:?}", err),
            _ => (),
        },
    }
}

fn main() {
    let mut rl = rustyline::Editor::<()>::new();
    for readline in rl.iter("> ") {
        match readline {
            Ok(line) => {
                parse_and_execute(&line);
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
