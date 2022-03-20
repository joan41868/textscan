#[derive(Debug)]
pub struct CommandLineArguments{
    pub file: String,
    pub query: String,
    pub skip_lines: bool
}

impl CommandLineArguments{
    pub fn new(args: Vec<String>) -> CommandLineArguments{
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
