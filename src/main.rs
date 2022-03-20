mod args;
mod line;

use std::env;
use std::fs;
use std::fmt;
use args::{CommandLineArguments};
use line::{Line};


fn compose_final_result(arr: &Vec<Line>, skip_lines: bool) -> String{
    let mut final_string: String = "".to_owned();
    let mut skip_lines_final = "".to_owned();
    for i in arr{
        if i.has_query{
            skip_lines_final.push_str(&i.stringify());
     }
     final_string.push_str(&i.stringify());
    }
    if skip_lines {
        skip_lines_final.to_string()
    }else{
        final_string.to_string()
    }
}

fn mapper(s :&str) -> String {
    s.to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("Not enough args. Given only {:?}, need file, query", args)
    }

    let cmd_args = CommandLineArguments::new(args);
    println!("Searching for \x1b[33m{}\x1b[0m in file \x1b[33m{}\x1b[0m", cmd_args.query, cmd_args.file);
    let file_contents = fs::read_to_string(cmd_args.file).expect("Something went wrong with file reading, probably file is missing");

    let splitted_by_newline: Vec<String> = file_contents.split('\n').map(mapper).collect();
    let mut line_arr :Vec<Line> = Vec::new();
    let mut line_counter = 1;

    
    for el in splitted_by_newline.iter(){
        
        let mut contains_query: bool = false;
        if el.contains(cmd_args.query.as_str()){
            contains_query = true;
        }
        
        let line = Line::new(line_counter, String::from(el), contains_query);
        let colorized_line = line.colorize_query_word(cmd_args.query.as_str());
        line_arr.push(Line::new(line_counter, colorized_line.line_content, contains_query));
    
        line_counter = line_counter + 1;
    
    }
    
    println!("{}",compose_final_result(&line_arr, cmd_args.skip_lines));
}

   