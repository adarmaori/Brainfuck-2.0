use std::cmp::max;
use std::fs;
use std::io::Read;
use std::path::Path;
use regex::Regex;

fn main() {
    let code = "examples/primes/main.bf";
    // let code = "test.bf";
    let _tape = evaluate(compile(code));
}

fn evaluate(code: String) -> Vec<u8> {
    let mut tape: Vec<u8> = vec![0; 30000];
    let mut ptr: usize = 0;
    let mut pc: usize = 0;
    let mut loop_stack = Vec::new();

    while pc < code.len() {
        let command = code.chars().nth(pc).unwrap();
        // match case for commands
        match command {
            '>' => ptr += 1,
            '<' => ptr -= 1,
            '+' => tape[ptr] += 1,
            '-' => tape[ptr] -= 1,
            '.' => print!("{}", tape[ptr] as char),
            ',' => tape[ptr] = std::io::stdin().bytes().next().unwrap().unwrap(),
            '[' => {
                if tape[ptr] == 0 {
                    let mut loop_count = 1;
                    while loop_count > 0 {
                        pc += 1;
                        let command = code.chars().nth(pc).unwrap();
                        if command == '[' {
                            loop_count += 1;
                        } else if command == ']' {
                            loop_count -= 1;
                        }
                    }
                } else {
                    loop_stack.push(pc as u32);
                }
            },
            ']' => {
                if tape[ptr] != 0 {
                    pc = loop_stack.pop().unwrap() as usize;
                }
                else {
                    loop_stack.pop();
                }
            },
            _ => {}
        }
        if command != ']' || tape[ptr] == 0{
            pc += 1;
        }
        let last = tape.iter().enumerate().rev()
            .find_map(|(index, &value)| if value != 0 { Some(index) } else { None });
        let mut last_index: usize;
        if last == None {
            last_index = 0;
        } else {
            last_index = last.unwrap();
        }
        last_index = max(last_index, ptr) + 1;
        for (index, &num) in tape.iter().enumerate() {
            if index == ptr {
                // Highlight this number
                print!("\x1b[32m{}\x1b[0m ", num); // Green color
            } else if index < last_index{
                // Print normally
                print!("{} ", num);
            }
        }
        print!("| {command} | pc: {pc}");
        println!();
    }
    tape
}

fn clean(code: &str) -> String {
    let mut clean_code = String::new();
    let mut last_seen = 0;
    for (ind, command) in code.chars().enumerate() {
        if ind < last_seen {
            continue;
        }
        else {
            last_seen = ind;
        }
        match command {
            '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' | '}' | ')'=> clean_code.push(command),
            '{' => {
                // get the text between this and the next }
                let mut i = ind;
                while code.chars().nth(i) != Some('}') {
                    clean_code.push(code.chars().nth(i).unwrap());
                    last_seen = i;
                    i += 1;
                }
            },
            '(' => {
                // get the text between this and the next )
                clean_code.push(command);
                let mut i = ind;
                let mut count = 1;
                while count > 0 {
                    i += 1;
                    if code.chars().nth(i) == Some('(') {
                        count += 1;
                    } else if code.chars().nth(i) == Some(')') {
                        count -= 1;
                    }
                    if count == 0 {
                        break;
                    }
                    clean_code.push(code.chars().nth(i).unwrap());
                }
                last_seen = i;
            },
            _ => {}
        }
    }
    clean_code
}

fn compile(file: &str) -> String {
    let mut path = file.to_string();
    if path.starts_with("/") {
        path.remove(0);
    }
    let mut code = fs::read_to_string(path).expect("Something went wrong reading the file");
    let dir = Path::new(file).parent().expect("Could not get parent directory").display().to_string();
    compile_plain(&code, dir)
}


fn compile_plain(code: &str, dir: String) -> String {
    let code = clean(&code);
    let mut compiled_code = String::new();
    let mut last_seen = 0;
    for (ind, command) in code.chars().enumerate() {
        if ind < last_seen {
            continue;
        }
        else {
            last_seen = ind;
        }
        match command {
            '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' => compiled_code.push(command),
            '{' => {
                // get the text between this and the next }
                let mut inset_name = String::new();
                let mut pc = ind + 1;
                while code.chars().nth(pc) != Some('}') {
                    inset_name.push(code.chars().nth(pc).unwrap());
                    last_seen = pc;
                    pc += 1;
                }
                println!("inserting code from file: {}", inset_name);
                // get the inset code from the filename [insert_name].bf
                let inset_file = format!("{dir}/{inset_name}.bf");
                // compile the inset code
                let inset_compiled_code = compile(&inset_file);
                // add the compiled inset code to the compiled code
                compiled_code.push_str(&inset_compiled_code);
            },
            '(' => {
                // get the text between this and the next )
                let mut num_and_action = String::new();
                let mut pc = ind + 1;
                while code.chars().nth(pc) != Some(')') {
                    num_and_action.push(code.chars().nth(pc).unwrap());
                    pc += 1;
                }
                last_seen = pc;
                // Find the number in the text
                let re = Regex::new(r"\d+").unwrap();
                let num = re.find(&num_and_action).unwrap().as_str().parse::<usize>().unwrap();
                // Find the rest
                let action = &num_and_action[num.to_string().len()..];
                let compiled_action = compile_plain(action, dir.clone());
                // Add the action num times
                for _ in 0..num {
                    compiled_code.push_str(&compiled_action);
                }
            }
            _ => {}
        }
    }
    println!("compiled code: {}", compiled_code);
    compiled_code
}