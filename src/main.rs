// Nerd Font required
use std::{
    env::args,
    fs::{read_to_string, write},
    process::exit,
};

const FILE_PATH: &str = "/home/dartt0n/.config/conky/todo.txt";
const DONE: &str = "";
const NOT_DONE: &str = "";

#[derive(std::cmp::PartialEq)]
enum Mode {
    Error,
    Add,
    Complete,
    Uncomplete,
    Remove,
    Help,
}

struct Task {
    done: bool,
    text: String,
}

fn print_help() {
    println!(
        "\
    Args:
    done | complete | d | c     [N]     Marking task number N as completed
    undone | uncomplete | u     [N]     Marking task number N as uncompleted
    add | new | a | n           [TEXT]  Adding new test with text TEXT
    remove | delete | rm | del  [N]     Removing task number N (also move other)\n\
    Examples:
    todo done 1
    
    todo undone 2
    
    todo add Phone Tom
    todo new Clear room
    
    todo remove 2
    todo delete 1
    "
    );
}

fn main() {
    let args = args().collect::<Vec<String>>()[1..].to_owned();
    // parsing args and changing mode
    let mode = match args[0].as_str() {
        "done" | "complete" | "d" | "c" => Mode::Complete,
        "undone" | "uncomplete" | "u" => Mode::Uncomplete,
        "add" | "new" | "a" | "n" => Mode::Add,
        "remove" | "delete" | "rm" | "del" => Mode::Remove,
        "-h" | "--help" | "help" => Mode::Help,
        _ => Mode::Error,
    };
    // reading all lines from file
    let buff = read_to_string(FILE_PATH).expect("Failed to read from file");
    // spliting it to array
    let lines: Vec<&str> = buff.split("\n").collect();

    // lines.remove(lines.len()-1);

    let mut todo_list: Vec<Task> = Vec::with_capacity(lines.len());
    // parsing every line
    for line in lines {
        if line == "" {
            break;
        }
        let words: Vec<&str> = line.split(" ").collect();
        let number: usize = {
            let x = words[0]; // "[x]"
            x[1..x.len() - 1].to_string().parse().expect("Failed to parse number as usize")
            // x
        };
        let is_done = words[1].to_string();
        let task = words[2..].join(" ");
        // saving
        todo_list.insert(
            number - 1,
            Task {
                done: is_done == DONE,
                text: task,
            },
        );
    }

    // different actions depending on mode
    if args.len() != 2 && mode != Mode::Add {
        println!("Wrong count of arguments");
        print_help();
        exit(1);
    }
    match mode {
        Mode::Error => {
            println!("Unknown parameter {}", args[0]);
            print_help();
            exit(1);
        }
        Mode::Help => {
            print_help();
            exit(0);
        }
        Mode::Add => {
            let text = args[1..].join(" ");
            todo_list.push(Task { done: false, text });
        }
        Mode::Remove => {
            let index: usize = args[1].parse().expect("N must me integer");
            todo_list.remove(index - 1);
        }
        Mode::Complete => {
            let index: usize = args[1].parse().expect("N must me integer");
            todo_list[index - 1].done = true;
        }
        Mode::Uncomplete => {
            let index: usize = args[1].parse().expect("N must me integer");
            todo_list[index - 1].done = false;
        }
    }

    let mut generated_text: String = String::new();

    // generating text and writing it to file
    for (index, task) in todo_list.into_iter().enumerate() {
        generated_text += &format!("[{}] {} {}\n", index + 1, if task.done { DONE } else { NOT_DONE }, task.text);
    }
    write(FILE_PATH, generated_text.as_bytes()).expect("Failed to write to file");
}
