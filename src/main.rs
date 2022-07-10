use std::collections::HashMap;
use std::fs::*;
use console::*;

const empty_string: String = String::new();
static mut loaded: (String, String, f32, bool) = (empty_string, empty_string, 0f32, false);
static mut i: i32 = 0i32;

fn main() {
    let file = read_to_string("/home/andreas/tankasm/test/test.tasm").unwrap_or("".to_string());

    let mut regs: HashMap<String, (String, String, f32, bool)> =  HashMap::new();

    let lns = (&file).lines().count() as i32;

    unsafe {
        while i < lns {
            if i < 0 {
                i = 0;
            }
            run(parse(String::from(file.lines().nth(i as usize).unwrap())), &mut regs);
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

    let template = &("s".to_owned(), "".to_owned(), 0f32, false);

    unsafe {
        match cmd {
            "prt" => prt(&mut loaded),
            "read" => {
                let stdin = Term::stdout();
                loaded = ("s".to_owned(), format!("{}", stdin.read_char().unwrap_or(' ')), 0f32, false)
            }
            "load" => { loaded = regs.get(&args[0]).unwrap_or(&("".to_owned(), "".to_owned(), 0f32, false)).to_owned() },
            "mov" => mov((&args[0]).to_owned(), (&args[1]).to_owned(), regs),
            "dmov" => mov((&loaded.1).to_owned(), (&args[0]).to_owned(), regs),
            "add" => {loaded = ((&loaded.0).to_owned(), (&loaded.1).to_owned(), loaded.2 + f32parse((&args[0]).to_owned()), loaded.3)},
            "sub" => {loaded = ((&loaded.0).to_owned(), (&loaded.1).to_owned(), loaded.2 - f32parse((&args[0]).to_owned()), loaded.3)},
            "mul" => {loaded = ((&loaded.0).to_owned(), (&loaded.1).to_owned(), loaded.2 * f32parse((&args[0]).to_owned()), loaded.3)},
            "div" => {loaded = ((&loaded.0).to_owned(), (&loaded.1).to_owned(), loaded.2 / f32parse((&args[0]).to_owned()), loaded.3)},
            "conc" => conc((&args).to_owned(), regs, regs.get(&args[0]).unwrap_or(template).to_owned()),
            "inst" => {regs.insert((&args[0]).to_owned(), (&loaded).to_owned());},
            "goto" => { i = f32parse((&args[0]).to_owned()) as i32 - 2 },
            "if" => process_if(args, regs),
            "lnb" => println!(),
            "flsh" => { regs.remove_entry(&args[0]); },
            _ => {}
        }
    }
}

fn process_if(args: Vec<String>, regs: &mut HashMap<String, (String, String, f32, bool)>) {
    let lines = f32parse((&args[1]).to_owned()) as i32;
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

fn conc(args: Vec<String>, regs:  &mut HashMap<String, (String, String, f32, bool)>, prev: (String, String, f32, bool)) {
    unsafe { regs.insert((&args[0]).to_owned(), (prev.0, format!("{}{}", prev.1, &loaded.1), prev.2, prev.3)); }
}

fn prt(var: &mut (String, String, f32, bool)) {
	match (var.0).as_str() {
		"s" => print!("{}", var.1),
		"n" => print!("{}", var.2),
		"b" => print!("{}", var.3),
		_ => ()
	}
}

fn mov(name: String, value: String, regs: &mut HashMap<String, (String, String, f32, bool)>) {
    let var_type = name.chars().nth(name.chars().position(|c| !"0123456789".contains(c)).unwrap()).unwrap();
	match var_type {
		's' => {
			regs.insert(name, ("s".to_string(), value, 0f32, false));
		},
		'n' => {
			regs.insert(name, ("n".to_string(), "".to_string(), f32parse(value), false));
		},
		'b' => {
			regs.insert(name, ("b".to_string(), "".to_string(), 0f32, value == String::from("true")));
		},
		_ => panic!()
	}
}

fn f32parse(num: String) -> f32 {
    let decimal = num.chars().position(|c| c == '.');
    let dec_point = match decimal {
        None => Option::from(num.len()),
        _ => decimal   
    }.unwrap();
    let mut val = 0.0f32;
    if dec_point != num.len() {
        for (j, c) in num[dec_point+1..].chars().enumerate() {
            val += char_to_num(c) * ((0.1f32).powf((j as f32)+1.0))
        }
    }
    for (j, c) in num[..dec_point].chars().rev().enumerate() {
        val += char_to_num(c) * ((10f32).powf(j as f32))
    }
    return val;
}

fn char_to_num(char: char) -> f32 {
    return match char {
        '0' => 0.0,
        '1' => 1.0,
        '2' => 2.0,
        '3' => 3.0,
        '4' => 4.0,
        '5' => 5.0,
        '6' => 6.0,
        '7' => 7.0,
        '8' => 8.0,
        '9' => 9.0,
        _   => 0.0,
    }
}