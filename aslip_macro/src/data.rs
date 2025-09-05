// aslip_macro

use crate::FnInfo;
use std::collections::HashMap;
use std::sync::Mutex;
extern crate proc_macro;

pub fn insert(info: FnInfo, span: proc_macro2::Span) -> Result<(), syn::Error> {
    let re = COMMANDS.lock();

    let key = info.gen_key();

    return match re {
        Ok(mut m) => {
            let _ = m.insert(key, info.clone());

            Ok(())
        }
        Err(e) => Err(syn::Error::new(span, e.to_string())),
    };
}

// let mut m = COMMANDS.lock().unwrap();
// let sadf = m.insert(String::new(), info);
// m.get(&String::new());
pub static COMMANDS: once_cell::sync::Lazy<Mutex<HashMap<String, FnInfo>>> =
    once_cell::sync::Lazy::new(|| Mutex::new(HashMap::new()));
