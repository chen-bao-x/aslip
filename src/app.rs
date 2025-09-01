pub struct App<'a> {
    pub _description: &'a str,

    // state
    pub _user_inputed_cmd_name: Option<String>,

    /// 如果没有传入参数，则等于 vec![]
    pub _user_inputed_cmd_args: Vec<String>,
}

impl<'a> App<'a> {
    pub fn new(descrip: &'a str) -> Self {
        let env_args: Vec<String> = std::env::args().collect();

        // env_args.get(0); // the exe file path.
        let user_inputed_cmd_name = env_args.get(1).map(|x| x.to_string()); // the sub_command name.

        // 第 2 个以及后面的所有.
        let user_inputed_cmd_args: Vec<String> = {
            if env_args.len() > 2 {
                env_args[2..].to_vec()
            } else {
                vec![]
            }
        };

        return Self {
            _description: descrip,
            _user_inputed_cmd_args: user_inputed_cmd_args,
            _user_inputed_cmd_name: user_inputed_cmd_name,
        };
    }

    pub fn run(&self) {
        let Some(cmd_name) = &self._user_inputed_cmd_name else {
            self.print_app_help();
            return;
        };
    }
}

impl<'a> App<'a> {
    fn print_app_help(&self) {
        println!("{}", "printing help document...");
    }
}
