// use aslip::*;

#[allow(unused_variables)]
pub mod ok_case {
    use std::ffi::*;
    use std::net::*;
    use std::path::PathBuf;

    use aslip::command;
    use aslip::types::NumberInRange;
    use aslip::types::OnOff;

    #[aslip::command]
    pub fn a2() {
        println!("ok")
    }

    #[command]
    /// 测试没有参数的命令。
    pub fn no_arg_action() {
        println!("ok")
    }

    #[command]
    pub fn a3452() {
        println!("ok")
    }

    /// 只有一个参数 的命令。
    ///
    /// <green,bold>Usage</>: <cyan,bold>app</> <cyan,bold>one_arg</> <green,bold><<PATH>></>
    ///
    /// <green!>Args</>:
    ///     <<PATH>>:     the file path.
    /// Options:
    ///     -h          print this message.
    #[command]
    pub fn one_arg(number: usize) {
        println!("ok")
    }

    #[command]
    /// 一句话说明。
    pub fn two_arg(a: NumberInRange<0, 88>, b: String) {
        println!("ok")
    }

    #[command()]
    /// arg 9 about.
    pub fn arg_9(a1: u8, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8, a7: u8, a8: u8, a9: Vec<u8>) {
        println!("ok")
    }

    #[command]
    pub fn collection_arg(first: u8, last: Vec<String>) {
        println!("ok")
    }

    #[command(name = "renamed", short = "c")]
    /// 测试 command(name = "renamed"）是否能定义命令名称。
    pub fn test_for_rename() {
        println!("ok")
    }

    /// 函数的文档注释的第一句话会用作 命令 的 <bold>一句话说明</bold> 打印到 cli 程序的帮助信息中。
    /// 整个帮助文档会被作为这个 命令 的 quick help 信息 ———— `app cmd -h` 时显示的信息。
    /// 还支持一些彩色标记语法哦： <red>red</red>  <bg:green>asdfadsf</>
    ///
    #[command]
    pub fn cmd_doc_example() {
        println!("ok")
    }

    #[command(name = "-h")]
    /// 测试 替换掉默认的 "-h" 实现。
    pub fn asdfadsfcmd_doc_example() {
        println!("ok")
    }

    #[command]
    #[allow(non_snake_case)]
    pub fn arg_NumberInRange(arg: NumberInRange<3, 9>) {
        let _ = arg;
        println!("ok")
    }

    #[command]
    #[allow(non_snake_case)]
    pub fn arg_OnOff(String: OnOff) {
        let _ = String;
        println!("ok")
    }
    #[command]
    #[allow(non_snake_case)]
    pub fn arg_String(String: String) {
        let _ = String;
        println!("ok")
    }

    #[command]
    pub fn arg_bool(bool: bool) {
        let _ = bool;
        println!("ok")
    }

    #[command]
    pub fn arg_char(char: char) {
        let _ = char;
        println!("ok")
    }

    #[command]
    pub fn arg_i8(i8: i8) {
        let _ = i8;
        println!("ok")
    }

    #[command]
    pub fn arg_u8(u8: u8) {
        let _ = u8;
        println!("ok")
    }

    #[command]
    pub fn arg_i16(i16: i16) {
        let _ = i16;
        println!("ok")
    }

    #[command]
    pub fn arg_u16(u16: u16) {
        let _ = u16;
        println!("ok")
    }

    #[command]
    pub fn arg_i32(i32: i32) {
        let _ = i32;
        println!("ok")
    }

    #[command]
    pub fn arg_u32(u32: u32) {
        let _ = u32;
        println!("ok")
    }

    #[command]
    pub fn arg_f32(f32: f32) {
        let _ = f32;
        println!("ok")
    }

    #[command]
    pub fn arg_i64(i64: i64) {
        let _ = i64;
        println!("ok")
    }

    #[command]
    pub fn arg_u64(u64: u64) {
        let _ = u64;
        println!("ok")
    }

    #[command]
    pub fn arg_f64(f64: f64) {
        let _ = f64;
        println!("ok")
    }

    #[command]
    pub fn arg_i128(i128: i128) {
        let _ = i128;
        println!("ok")
    }

    #[command]
    pub fn arg_u128(u128: u128) {
        let _ = u128;
        println!("ok")
    }

    #[command]
    pub fn arg_isize(isize: isize) {
        let _ = isize;
        println!("ok")
    }

    #[command]
    pub fn arg_usize(usize: usize) {
        let _ = usize;
        println!("ok")
    }
    #[allow(non_snake_case)]
    #[command]
    pub fn arg_CString(CString: CString) {
        let _ = CString;
        println!("ok")
    }

    #[command]
    #[allow(non_snake_case)]
    pub fn arg_OsString(OsString: OsString) {
        let _ = OsString;
        println!("ok")
    }

    #[command]
    #[allow(non_snake_case)]
    pub fn arg_IpAddr(IpAddr: IpAddr) {
        let _ = IpAddr;
        println!("ok")
    }
    #[allow(non_snake_case)]
    #[command]
    pub fn arg_Ipv4Addr(Ipv4Addr: Ipv4Addr) {
        let _ = Ipv4Addr;
        println!("ok")
    }

    #[command]
    #[allow(non_snake_case)]
    pub fn arg_Ipv6Addr(Ipv6Addr: Ipv6Addr) {
        let _ = Ipv6Addr;
        println!("ok")
    }

    #[command]
    #[allow(non_snake_case)]
    pub fn arg_SocketAddr(SocketAddr: SocketAddr) {
        let _ = SocketAddr;
        println!("ok")
    }

    #[command]
    #[allow(non_snake_case)]
    pub fn arg_SocketAddrV4(SocketAddrV4: SocketAddrV4) {
        let _ = SocketAddrV4;
        println!("ok")
    }

    #[command]
    #[allow(non_snake_case)]
    pub fn arg_SocketAddrV6(SocketAddrV6: SocketAddrV6) {
        let _ = SocketAddrV6;
        println!("ok")
    }

    #[command]
    #[allow(non_snake_case)]
    pub fn arg_PathBuf(PathBuf: PathBuf) {
        let _ = PathBuf;
        println!("ok")
    }

    //------------------- # error case: 这几个测试是用来测试 实现的 aslip_macro 是否能正常报错的。

    // /// 测试规则：command 函数不能有返回值。
    // #[command]
    // pub fn no_returns_rule_test() -> u8 {
    //     return 3;
    // }

    // /// 测试规则：只有最后一个参数可以使用 Vec<T> 类型。
    // #[command]
    // pub fn only_last_arg_can_vec_rule_test(a: Vec<String>, a: u8) {}

    // /// 测试 rule 3. 命令的名称不能重复。
    // #[command]
    // pub fn no_arg_adction() {
    //     println!("{}", module_path!());
    //     println!("I'm no_arg_action, happy to see you.");
    // }
}
