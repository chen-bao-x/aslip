// aslip_macro

use std::collections::HashMap;
use std::sync::Mutex;
use crate::FnInfo;
extern crate proc_macro;

pub fn insert(info: FnInfo, span: proc_macro2::Span) -> Result<(), syn::Error> {
    let re = COMMANDS.lock();

    match re {
        Ok(mut m) => {
            let _old_cmd = m.insert(info.func_name.clone(), info.clone());
            // crate::rule_3(old_cmd, info, span)?; // 这是重复名称检查的实现有 bug，暂时先不用。

            return Ok(());
        }
        Err(e) => Err(syn::Error::new(span, e.to_string())),
    }
}

// let mut m = COMMANDS.lock().unwrap();
// let sadf = m.insert(String::new(), info);
// m.get(&String::new());
pub static COMMANDS: once_cell::sync::Lazy<Mutex<HashMap<String, FnInfo>>> =
    once_cell::sync::Lazy::new(|| Mutex::new(HashMap::new()));
