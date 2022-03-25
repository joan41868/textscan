#[derive(Debug)]
pub struct CommandLineArguments{
    pub file: String,
    pub query: String,
    pub skip_lines: bool
}

impl CommandLineArguments{
    pub fn new(args: Vec<String>) -> CommandLineArguments{
        let arg_len = args.len();
        if arg_len < 4 {
            panic!("Not enough args. Given only {:?}, need file, query", args)
        }

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
            let k = key.trim();
            match k {
                "-f" => cmd_args.file = String::from(val),
                "-q" => cmd_args.query = String::from(val),
                _ => continue
            }
        }
        cmd_args.skip_lines = args.contains(&String::from("--skip"));
        if cmd_args.file.is_empty() {
            panic!("missing file")
        }
        if cmd_args.query.is_empty() {
            panic!("missing query")
        }
        cmd_args
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn constructs_as_expected() {
        let mut input:Vec<String> = Vec::new();
        input.push(String::from("textscan"));
        input.push(String::from("-q"));
        input.push(String::from("let"));
        input.push(String::from("-f"));
        input.push(String::from("file"));
        input.push(String::from("--skip"));
        let mut args = CommandLineArguments::new(input.clone());
        assert_eq!(args.file, "file");
        assert_eq!(args.skip_lines, true);
        assert_eq!(args.query, "let");

        input.pop();
        args = CommandLineArguments::new(input);
        assert_eq!(args.file, "file");
        assert_eq!(args.skip_lines, false);
        assert_eq!(args.query, "let");
    }

    #[test]
    #[should_panic]
    fn fails_on_missing_query() {
        let mut args_input:Vec<String> = Vec::new();
        args_input.push(String::from("-f"));
        args_input.push(String::from("file"));
        args_input.push(String::from("file"));
        args_input.push(String::from("file"));
        CommandLineArguments::new(args_input.clone());
    }

    #[test]
    #[should_panic]
    fn fails_on_missing_file() {
        let mut args_input:Vec<String> = Vec::new();
        args_input.push(String::from("-q"));
        args_input.push(String::from("let"));
        args_input.push(String::from("let"));
        args_input.push(String::from("let"));
        CommandLineArguments::new(args_input.clone());
    }

    #[test]
    #[should_panic]
    fn fails_on_low_args_len() {
        let mut args_input:Vec<String> = Vec::new();
        args_input.push(String::from("-f"));
        args_input.push(String::from("file"));
        CommandLineArguments::new(args_input.clone());
    }

}
