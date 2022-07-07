mod cmds;
use std::collections::HashMap;
use std::any::Any;
use std::fs::*;
use cmds::*;

const empty_string: String = String::new();
static mut loaded: (String, String, f32, bool) = (empty_string, empty_string, 0f32, false);
static mut i: usize = 0;

fn main() {
    let file = read_to_string("E:\\Projects\\tankasm\\test\\test.tasm").unwrap_or("".to_string());

    let mut regs: HashMap<String, (String, String, f32, bool)> =  HashMap::new();

    let lns = (&file).lines().count();

    unsafe {
        while i < lns {
            run(parse(String::from(file.lines().nth(i).unwrap())), &mut regs);
            i+=1;
        }
    }
}

fn parse(inp: String) -> Vec<String> {
    let mut ret_val: Vec<String> = Vec::new();

    let mut in_str = false;
    let mut buf = String::new();

    for c in inp.chars() {
        if c == '"' {
            in_str = !in_str;

            if !in_str {
                ret_val.push((&buf).to_owned());
                buf = String::new();
            }
        }
        else if c == ' ' && !in_str {
            ret_val.push((&buf).to_owned());
            buf = String::new();
        }
        else {
            buf.push(c);
        }
    }
    ret_val.push((&buf).to_owned());

    ret_val
}

fn run(inp: Vec<String>, regs: &mut HashMap<String, (String, String, f32, bool)>) {
    let cmd = &inp[0].to_string();
    let cmd = cmd.as_str();
    let args = inp[1..].to_vec();

    unsafe {
        match cmd {
            "prt" => prt(&mut loaded),
            "load" => {if regs.contains_key(&args[0]) { loaded = regs.get(&args[0]).unwrap().to_owned() } },
            "mov" => mov((&args[0]).to_owned(), (&args[1]).to_owned(), regs),
            "add" => {loaded = ((&loaded.0).to_owned(), (&loaded.1).to_owned(), loaded.2 + f32parse((&args[0]).to_owned()), loaded.3)},
            "inst" => {regs.insert((&args[0]).to_owned(), (&loaded).to_owned());},
            "goto" => { i = f32parse((&args[0]).to_owned()) as usize - 2 },
            "if" => process_if(args, regs),
            _ => ()
        }
    }
}

fn process_if(args: Vec<String>, regs: &mut HashMap<String, (String, String, f32, bool)>) {
    let lines = f32parse((&args[1]).to_owned()) as usize;
    let statement = (&args[0]).to_owned();

    if !eval(statement, regs) {
        unsafe { i += lines }
    }
}

fn eval(inp: String, regs: &mut HashMap<String, (String, String, f32, bool)>) -> bool {
    let comparison;
    let comp_char;
    if inp.contains(">=") {
        comparison = inp.split(">=").collect::<Vec<&str>>();
        comp_char = ">=";
    }
    else if inp.contains("<=") {
        comparison = inp.split("<=").collect::<Vec<&str>>();
        comp_char = "<=";
    }
    else if inp.contains("==") {
        comparison = inp.split("==").collect::<Vec<&str>>();
        comp_char = "==";
    }
    else if inp.contains("!=") {
        comparison = inp.split("!=").collect::<Vec<&str>>();
        comp_char = "!=";
    }
    else if inp.contains(">") {
        comparison = inp.split(">").collect::<Vec<&str>>();
        comp_char = ">";
    }
    else if inp.contains("<") {
        comparison = inp.split("<").collect::<Vec<&str>>();
        comp_char = "<";
    }
    else {
        return false;
    }

    match comp_char {
        "==" => {
            return regs.get(comparison[0]).unwrap() == regs.get(comparison[1]).unwrap()
        },
        "!=" => {
            return regs.get(comparison[0]).unwrap() != regs.get(comparison[1]).unwrap()
        },
        ">=" => {
            return regs.get(comparison[0]).unwrap().0 == "n" && regs.get(comparison[1]).unwrap().0 == "n" && regs.get(comparison[0]).unwrap() >= regs.get(comparison[1]).unwrap()
        },
        "<=" => {
            return regs.get(comparison[0]).unwrap().0 == "n" && regs.get(comparison[1]).unwrap().0 == "n" && regs.get(comparison[0]).unwrap() <= regs.get(comparison[1]).unwrap()
        },
        ">" => {
            return regs.get(comparison[0]).unwrap().0 == "n" && regs.get(comparison[1]).unwrap().0 == "n" && regs.get(comparison[0]).unwrap() > regs.get(comparison[1]).unwrap()
        },
        "<" => {
            return regs.get(comparison[0]).unwrap().0 == "n" && regs.get(comparison[1]).unwrap().0 == "n" && regs.get(comparison[0]).unwrap() < regs.get(comparison[1]).unwrap()
        }
        _ => false
    }
}