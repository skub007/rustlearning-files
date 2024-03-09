use std::{io, process};
use colored::*;
use crossterm::terminal::*;
fn main() {
    println!("{}",1+1*2);
    Clear(ClearType::All);
    let mut r: f32 = 0.0;
    let mut v = false;
    println!("{}", v);
    let mut value = String::new();
    let mut op = String::new();
    println!("{}","PLease give the first Number And Operation-Symbol for\nChecking And Setting up Env ->".red());
    io::stdin()
        .read_line(&mut value)
        .expect("Expected A Number");
    loop {
        if value.trim().is_empty() {
            println!("Number ->");
            io::stdin()
                .read_line(&mut value)
                .expect("Expected A Number");
            if value.trim() == "q" || value.trim() == "quit" {
                endman(r);
            }
            let num: Result<f32, _> = value.trim().parse();
            match num {
                Ok(_) => {
                    op.clear();
                    continue;
                }
                Err(_) => {
                    v = true;
                    println!("Err : {}", v);
                    diderr();
                    break;
                }
            }
        } else {
            println!("Operation Symbol ->");
            io::stdin()
                .read_line(&mut op)
                .expect("Expected A Operation Symbol");
            r = performop(r, &op, &value);
            value.clear();
        }
    }
}

fn endman(r: f32) {
    println!("Result Is : {}", r);
    let mut o = String::new();
    io::stdin().read_line(&mut o).unwrap();
    process::exit(0x0100);
}

fn performop(mut r: f32, op: &String, value: &String) -> f32 {
    let trimv = value.trim().parse::<f32>().expect("err");
    match op.trim() {
        "+" => r += trimv,
        "-" => r -= trimv,
        "*" => r *= trimv,
        "/" => r /= trimv,
        "q" | "quit" => {
            endman(r);
        }
        _ => diderr(),
    }
    r
}

fn diderr() {
    println!("{}","ERR - NUMBER OR ARITHMETC SYMBOL EXPECTED....".yellow());
    let mut o = String::new();
    io::stdin().read_line(&mut o).unwrap();
    process::exit(0x0100);
}
