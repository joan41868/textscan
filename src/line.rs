use std::fmt;

#[derive(Debug)]
pub struct Line{
    pub line_num: u128,
    pub line_content: String,
    pub has_query: bool
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