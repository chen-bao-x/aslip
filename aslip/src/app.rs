use std::str;

use owo_colors::OwoColorize;

/// 单个命令的相关信息。
#[derive(Debug, Clone)]
pub struct CmdInfo<'a> {
    pub name: &'a str,
    pub short_name: &'a str,

    /// 命令的一句话说明。
    pub about: &'a str,

    pub quick_help: &'a str,

    /// `fn f(a: String)` 中的 ("a", "String").
    pub args: &'a [(&'a str, &'a str)],
}

impl<'a> CmdInfo<'a> {
    pub fn print_cmd_quick_help(&self, app_name: &str) -> String {
        if !self.quick_help.is_empty() {
            return self.quick_help.to_string();
        }

        let name = self.name.style(CMD_NAME);
        let about = self.about;
        let app_name = app_name.style(CMD_NAME);
        let args = self.format_args().style(ARG_TYPE).to_string();

        let usage = "Usage:".style(ARG_VALUE);

        // let args_ = "Arguments: ".style(ARG_VALUE);

        // let args_about = {
        //     let mut re = String::new();

        //     // for x in self.format_args().split_ascii_whitespace() {
        //     for x in self.args {
        //         re.push_str("    ");
        //         re.push_str(&x.0.style(ARG_TYPE).to_string());
        //         re.push_str("\t");
        //         re.push_str(&x.1);
        //         re.push_str("\t");
        //         re.push_str("类型说明");
        //         re.push_str("\n");
        //     }

        //     re
        // };
        format!(
            r###"
{about}

{usage} 
    {app_name} {name} {args}

            "###
        )
    }

    pub const fn new(
        name: &'a str,
        short_name: &'a str,
        about: &'a str,
        args: &'a [(&'a str, &'a str)],
        quick_help: &'a str,
    ) -> Self {
        CmdInfo {
            name,
            short_name,
            about,
            args: args,
            quick_help: quick_help,
        }
    }

    fn format_args(&self) -> String {
        let mut re = String::new();
        for x in self.args {
            if is_colection_type(x.1) {
                re.push_str("[");
                re.push_str(x.0);
                re.push_str("...");
                re.push_str("] ");
            } else {
                re.push_str("<");
                re.push_str(x.0);

                re.push_str("> ");
            }
        }

        return re;

        fn is_colection_type(type_name: &str) -> bool {
            let collection_types = ["Vec", "HashSet", "BTreeSet", "VecDeque"];

            let type_name: &str = &type_name.replace(" ", "");

            for x in collection_types {
                if type_name.starts_with(x) {
                    return true;
                }
            }

            return false;
        }
    }
}

#[derive(Debug, Clone)]
/// 保存一些命令行程序的信息。
pub struct App<'a> {
    pub _app_name: String,
    pub _about: &'a str,
    pub _version: &'a str,
    pub _author: &'a str,

    /// app help 时显示的信息。
    pub _help: &'a str,

    pub _commands: Vec<CmdInfo<'a>>,
    // pub _commands: HashMap<&'a str, CmdInfo<'a>>,

    // state
    ///
    pub _user_inputed_cmd_name: Option<String>,

    /// 如果没有传入参数，则等于 vec![] ``
    pub _user_inputed_cmd_args: Vec<String>,
}

impl<'a> App<'a> {
    pub const fn const_new() -> Self {
        Self {
            _app_name: String::new(),
            _about: "",
            _version: "",
            _author: "",
            _help: "",
            _commands: Vec::new(),
            _user_inputed_cmd_name: Some(String::new()),
            _user_inputed_cmd_args: Vec::new(),
        }
    }
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
            _about: "",
            _user_inputed_cmd_name: user_inputed_cmd_name,
            _user_inputed_cmd_args: user_inputed_cmd_args,
            _version: env!("CARGO_PKG_VERSION"),
            // _commands: Vec::new(),
            _commands: Vec::new(),
            _author: "",
            _help: "",
        };
    }

    pub fn app_name(self, name: &str) -> Self {
        let mut re = self;
        re._app_name = name.to_string();
        return re;
    }
    pub const fn about(self, description: &'a str) -> Self {
        let mut re = self;
        re._about = description;
        return re;
    }
    pub const fn version(self, version: &'a str) -> Self {
        let mut re = self;
        re._version = version;
        return re;
    }
    pub const fn author(self, author: &'a str) -> Self {
        let mut re = self;
        re._author = author;
        return re;
    }
    pub const fn help(self, help: &'a str) -> Self {
        let mut re = self;
        re._help = help;
        return re;
    }

    /// app cmd -h 时调用的函数。
    pub fn print_cmd_quick_help_for(&self, cmd_name: &str) {
        let re = self
            ._commands
            .iter()
            .find(|x| x.name == cmd_name || x.short_name == cmd_name);

        // match self._commands.get(cmd_name) {
        match re {
            Some(info) => {
                println!("{}", info.print_cmd_quick_help(&self._app_name));
            }
            None => {
                eprintln!("Unkone Command: {cmd_name}")
            }
        };
    }
}

impl<'a> App<'a> {
    /// 当用户没有传入任何 命令 时， 会执行这个函数。
    pub fn print_app_help(&self) {
        if !self._help.is_empty() {
            return println!("{}", self._help);
        }

        let app_name = self._app_name.style(CMD_NAME);
        let description = self._about;
        let author = "\n".to_string() + self._author;

        let usage_marker = "Usage:".bold().green().to_string();
        let usage = format!("    {app_name} {}", "[Command]".style(ARG_TYPE));

        let commands_marker = "Commands:".bold().green().to_string();

        let command_list = {
            let mut re = String::new();
            for x in self._commands.iter() {
                // let x = kv_pair.1;

                re.push_str("    ");
                re.push_str(x.name);

                if !x.short_name.is_empty() {
                    re.push_str(", ");
                    re.push_str(x.short_name);
                }

                re.push_str("\t\t");
                re.push_str(x.about);
                re.push_str("\n");
            }

            re
        };

        let app_help_msg: String = format!(
            r###"
{description}{author}

{usage_marker}
{usage}

{commands_marker}
{command_list}

Use "{app_name} {h} {c}" for more information about a command.
        "###,
            h = "help".style(CMD_NAME),
            c = "<command>".style(ARG_TYPE)
        );

        println!("{}", app_help_msg);
    }
}

const ERR_MARKER: owo_colors::Style = owo_colors::style().bold().red();
const CMD_NAME: owo_colors::Style = owo_colors::style().bold().cyan();
const ARG_TYPE: owo_colors::Style = owo_colors::style().cyan();
const ARG_VALUE: owo_colors::Style = owo_colors::style().bold().green();
