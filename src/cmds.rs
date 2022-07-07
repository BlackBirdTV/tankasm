use std::any::{Any, TypeId, type_name};
use std::collections::HashMap;

pub fn prt(var: &mut (String, String, f32, bool)) {
	match (var.0).as_str() {
		"s" => println!("{}", var.1),
		"n" => println!("{}", var.2),
		"b" => println!("{}", var.3),
		_ => ()
	}
}

pub fn mov(name: String, value: String, regs: &mut HashMap<String, (String, String, f32, bool)>) {
	let var_type = name.chars().nth(1).unwrap();
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

pub fn f32parse(num: String) -> f32 {
    let decimal = num.chars().position(|c| c == '.');
    let dec_point = match decimal {
        None => Option::from(num.len()),
        _ => decimal   
    }.unwrap();
    let mut val = 0.0f32;
    if dec_point != num.len() {
        for (i, c) in num[dec_point+1..].chars().enumerate() {
            val += char_to_num(c) * ((0.1f32).powf((i as f32)+1.0))
        }
    }
    for (i, c) in num[..dec_point].chars().rev().enumerate() {
        val += char_to_num(c) * ((10f32).powf(i as f32))
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