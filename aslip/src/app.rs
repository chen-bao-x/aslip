use std::str;

use owo_colors::OwoColorize;

/// 单个命令的相关信息。
#[derive(Debug)]
pub struct CmdInfo<'a> {
    pub name: &'a str,
    pub short_name: &'a str,

    /// 命令的一句话说明。
    pub about: &'a str,
}

impl<'a> CmdInfo<'a> {
    pub fn usage(&self) -> String {
        todo!("生成 usage 的功能还未完成。")
    }

    pub const fn new(name: &'a str, short_name: &'a str, about: &'a str) -> Self {
        CmdInfo {
            name,
            short_name,
            about,
        }
    }
}

fn dsafadsf() {
    const asdf: &[CmdInfo] = &[
        CmdInfo {
            name: "todo!()",
            short_name: "",
            about: "todo!()",
        },
        CmdInfo::new("name", "short_name", "about"),
    ];
}

#[derive(Debug)]
/// 保存一些命令行程序的信息。
pub struct App<'a> {
    pub _app_name: String,
    pub _about: &'a str,
    pub _version: &'a str,

    pub _commands: Vec<CmdInfo<'a>>,

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
            _about: "",
            _user_inputed_cmd_name: user_inputed_cmd_name,
            _user_inputed_cmd_args: user_inputed_cmd_args,
            _version: env!("CARGO_PKG_VERSION"),
            _commands: Vec::new(),
        };
    }

    pub fn app_name(self, name: &str) -> Self {
        let mut re = self;
        re._app_name = name.to_string();
        return re;
    }
    pub fn about(self, description: &'a str) -> Self {
        let mut re = self;
        re._about = description;
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
        let app_name = self._app_name.style(CMD_NAME);
        let description = &self._about;

        let usage_marker = "Usage:".bold().green().to_string();
        let usage = format!("    {app_name} [Command]");

        let commands_marker = "Commands:".bold().green().to_string();

        let command_list = {
            let mut re = String::new();
            for x in &self._commands {
                re.push_str("    ");
                re.push_str(x.name);
                re.push_str(", ");
                re.push_str(x.short_name);
                
                re.push_str("\t\t");
                re.push_str(x.about);
                re.push_str("\n");
            }

            re
        };

        let app_help_msg: String = format!(
            r###"
{description}

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
