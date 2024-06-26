use std::env;
use std::fs;

mod token;
mod scanner;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let file = &args[1];
        read_file(file.to_string());
    } else {
        run_prompt()
    }
}

fn read_file(file: String) {
    let contents = fs::read_to_string(file).expect("File not found");
    run(contents);
}

fn run_prompt() {
    println!("Flora");

    loop {
        println!(">>> ");

        let mut input_user = String::new();
        std::io::stdin()
            .read_line(&mut input_user)
            .expect("can not read user input");

        run(input_user);
    }
}

fn run(source: String) {
    let tokens = source.split("\r\n");
    for token in tokens {
        println!("{:?}", token);
    }
}

fn error(line_code: i32, message: String) {
    report(line_code, String::new(), message)
}

fn report(line_code: i32, where_code: String, message: String) {
    println!("line = {} has the following error {} : {}", line_code, where_code, message);
}

