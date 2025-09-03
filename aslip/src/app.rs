//

#[derive(Debug)]
/// 保存一些命令行程序的信息。
pub struct App<'a> {
    pub _app_name: String,
    pub _description: &'a str,
    pub _version: &'a str,

    // state
    ///
    pub _user_inputed_cmd_name: Option<String>,

    /// 如果没有传入参数，则等于 vec![] ``
    pub _user_inputed_cmd_args: Vec<String>,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let env_args: Vec<String> = std::env::args().collect();

        // let executable_file_path env_args.get(0);
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
            _app_name: app_name,
            _description: "",
            _user_inputed_cmd_name: user_inputed_cmd_name,
            _user_inputed_cmd_args: user_inputed_cmd_args,
            _version: env!("CARGO_PKG_VERSION"),
        };
    }

    pub fn app_name(self, name: &str) -> Self {
        let mut re = self;
        re._app_name = name.to_string();
        return re;
    }
    pub fn description(self, description: &'a str) -> Self {
        let mut re = self;
        re._description = description;
        return re;
    }
    pub fn version(self, version: &'a str) -> Self {
        let mut re = self;
        re._version = version;
        return re;
    }
}

impl<'a> App<'a> {
    pub fn print_app_help(&self) {
        println!("{}", "printing help document...");
    }
}
