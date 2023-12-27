use std::cmp::max;
use std::io::Read;
fn main() {
    let code = "+++{duplicate/1}>+{duplicate/2}";
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
    for command in code.chars() {
        match command {
            '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' => clean_code.push(command),
            _ => {}
        }
    }
    clean_code
}

fn compile(code: &str) -> String {
    let mut compiled_code = String::new();
    for (ind, command) in code.chars().enumerate() {
        match command {
            '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' => compiled_code.push(command),
            '{' => {
                // get the text between this and the next }
                let mut inset_name = String::new();
                let mut pc = ind + 1;
                while code.chars().nth(pc) != Some('}') {
                    inset_name.push(code.chars().nth(pc).unwrap());
                    pc += 1;
                }
                println!("interting code from file: {}", inset_name);
                // get the inset code from the filename [insert_name].bf
                let mut inset_code = String::new();
                let mut inset_file = std::fs::File::open(format!("{}.bf", inset_name)).unwrap();
                inset_file.read_to_string(&mut inset_code).unwrap();
                // clean the inset code
                inset_code = clean(&inset_code);
                // compile the inset code
                let inset_compiled_code = compile(&inset_code);
                // add the compiled inset code to the compiled code
                compiled_code.push_str(&inset_compiled_code);
            }
            _ => {}
        }
    }
    println!("compiled code: {}", compiled_code);
    compiled_code
}