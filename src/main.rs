extern crate mylib;
use mylib::load_questions;
use std::io::stdin;
use std::io::BufRead;

fn main() {
    let questions = load_questions("invalid.txt").unwrap();
    println!("starting Questions!");

    let stdin = stdin();
    let mut std_lock = stdin.lock();
    let mut line_to_read : String = String::new();
    
    for question in questions {
        println!("{}", question.question_text());
        std_lock.read_line(&mut line_to_read);
        println!("{}",question.answer_text());

        println!("{}", question.check_answer(line_to_read.trim()).unwrap());
    }
}
