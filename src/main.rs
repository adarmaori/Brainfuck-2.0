use std::cmp::max;
use std::fs;
use std::io::Read;
use std::path::Path;

fn main() {
    let code = "examples/fibonacci/main.bf";
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
            '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' | '}' => clean_code.push(command),
            '{' => {
                // get the text between this and the next }
                let mut i = ind;
                while code.chars().nth(i) != Some('}') {
                    clean_code.push(code.chars().nth(i).unwrap());
                    last_seen = i;
                    i += 1;
                }
            }
            _ => {}
        }
    }
    clean_code
}

fn compile(file: &str) -> String {
    let mut code = fs::read_to_string(file).expect("Something went wrong reading the file");
    code = clean(&code);
    let dir = Path::new(file).parent().expect("Could not get parent directory").display().to_string();
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
            }
            _ => {}
        }
    }
    println!("compiled code: {}", compiled_code);
    compiled_code
}