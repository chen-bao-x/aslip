


expand:
    cargo expand -p aslip_test main >   "./target/expandes.rs"


 
test:
    RUSTFLAGS="-Awarnings" cargo run -q -p aslip_test -- 
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test --   no_arg_action
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test --   one_arg 3
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test --   two_arg 3 "this is text."
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test --   arg_9 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18


err_case: 
    -RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_bool "asdf"
    -RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_char "asdf"
    -RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_i8 "asdf"
    -RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_u8 "asdf"
    -RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_i16 "asdf"
    -RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_u16 "asdf"
    -RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_i32 "asdf"
    -RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_u32 "asdf"
    -RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_f32 "asdf"
    -RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_i64 "asdf"
    -RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_u64 "asdf"
    -RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_f64 "asdf"
    -RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_i128 "asdf"
    -RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_u128 "asdf"
    -RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_isize "asdf"
    -RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_usize "asdf"
    -RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_IpAddr "asdf"
    -RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_Ipv4Addr "asdf"
    -RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_Ipv6Addr "asdf"
    -RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_SocketAddr "asdf"
    -RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_SocketAddrV4 "asdf"
    -RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_SocketAddrV6 "asdf"
    -RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_PathBuf "$asdf"
    -RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_OnOff "asdf"
    -RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_NumberInRange "99"
    -RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_NumberInRange "1"
    -RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_NumberInRange "adsfdsa"
