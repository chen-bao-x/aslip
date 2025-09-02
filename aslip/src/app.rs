/// 保存一些命令行程序的信息。
pub struct App<'a> {
    pub _app_name: String,
    pub _description: &'a str,

    // TODO: app_document
    // TODO: command_document
    // TODO: argument description.

    // state
    ///
    pub _user_inputed_cmd_name: Option<String>,

    /// 如果没有传入参数，则等于 vec![]
    pub _user_inputed_cmd_args: Vec<String>,
}

impl<'a> App<'a> {
    pub fn new(descrip: &'a str) -> Self {
        let env_args: Vec<String> = std::env::args().collect();

        // env_args.get(0); // the exe file path.
        let user_inputed_cmd_name = env_args.get(1).map(|x| x.to_string()); // the sub_command name.

        let app_name = env_args
            .first()
            .cloned()
            .map(|path| {
                std::path::Path::new(&path)
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
            })
            .unwrap_or_default()
            .unwrap_or_default();

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
            _app_name: app_name,
        };
    }

    pub fn run(&self) {
        let Some(cmd_name) = &self._user_inputed_cmd_name else {
            self.print_app_help();
            return;
        };

        match cmd_name.as_str() {
            "" => {
                panic!("命令的名称不能时 空字符串 \"\"")
            }
            "no_arg_action" => {}
            _ => {}
        }
    }
}

impl<'a> App<'a> {
    pub fn print_app_help(&self) {
        println!("{}", "printing help document...");
    }
}
