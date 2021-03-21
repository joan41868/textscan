use std::env;
use std::fs;
use std::fmt;

#[derive(Debug)]
struct Line{
    line_num: u128,
    line_content: String,
    has_query: bool
}

impl fmt::Display for Line{
    fn fmt(&self,  f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "{} {}", self.line_num, self.line_content)
    }
}

impl Line{
    pub fn new(line_num:u128, line_content: String, has_query: bool) -> Self{
        Line{
            line_num,
            line_content,
            has_query
        }
    }

    pub fn stringify(&self) -> String{
        if self.has_query{
            format!("\x1b[34m{}\x1b[0m {}\n", self.line_num, self.line_content)
        }else{
            format!("{} {}\n", self.line_num, self.line_content)
        }
    }

    pub fn colorize_query_word(mut self, query: &str) -> Self{
        if self.has_query{
            let begining_idx = self.line_content.to_owned().find(query).unwrap();
            let ending_idx = begining_idx + query.len();
            let new_line_content = String::from(self.line_content);
            let new_line_content = 
                String::from(&new_line_content[0..begining_idx]) 
                + "\x1b[33m" 
                + &new_line_content[begining_idx..ending_idx] 
                + "\x1b[0m" + &new_line_content[ending_idx..];
            
            self.line_content = new_line_content;
            self
        }else{
            self
        }
    }
}

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

#[derive(Debug)]
struct CommandLineArguments{
    file: String,
    query: String,
    skip_lines: bool
}

impl CommandLineArguments{
    pub fn from(args: Vec<String>) -> CommandLineArguments{
        let arg_len = args.len();
        let mut cmd_args: CommandLineArguments = CommandLineArguments{
            query: String::from(""),
            file: String::from(""),
            skip_lines: false
        };
        
        for i in 0..arg_len{
            if i == arg_len - 1 {
                break;
            }
            let key = &args[i];
            let val = &args[i+1];
            if key == "-f"{
                cmd_args.file = String::from(val);
            }

            if key == "-q"{
                cmd_args.query = String::from(val);
            }
        }
        if args.contains(&String::from("--skip")){
            cmd_args.skip_lines = true
        }else{
            cmd_args.skip_lines = false
        }
        cmd_args
    }
}



fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 3{
        panic!("Not enough args")
    }

    let cmd_args = CommandLineArguments::from(args);
    println!("Searching for {} in file {}", cmd_args.query, cmd_args.file);
    let file_contents = fs::read_to_string(cmd_args.file).expect("Something went wrong with file reading, probably file is missing");

    let splitted_by_newline: Vec<&str> = file_contents.split('\n').collect();
    let mut line_arr :Vec<Line> = Vec::new();
    let mut line_counter = 1;

    
    for el in splitted_by_newline.iter(){
        
        let mut contains_query: bool = false;
        if el.contains(cmd_args.query.as_str()){
            contains_query = true;
        }
        
        let line = Line::new(line_counter, String::from(*el), contains_query);
        let colorized_line = line.colorize_query_word(cmd_args.query.as_str());
        line_arr.push(Line::new(line_counter, colorized_line.line_content, contains_query));
    
        line_counter = line_counter + 1;
    
    }
    
    println!("{}",compose_final_result(&line_arr, cmd_args.skip_lines));
}

   