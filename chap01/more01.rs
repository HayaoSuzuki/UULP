use std::env;
use std::io;

fn main() {
    let _args: Vec<String> = env::args().collect();

    if *&_args[1..].len() == 0 {
        // do_more(io::stdin())
        println!("arg = {}", "arg");
    } else {
        for arg in &_args[1..] {
            println!("arg = {}", arg);
        }
    }
}

fn do_more() {}

fn see_more() {}
