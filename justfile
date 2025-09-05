#! bin/bash


expand:
    cargo expand -p aslip_test main >   "./target/expandes.rs"


 
test:
    RUSTFLAGS="-Awarnings" cargo run -q -p aslip_test -- 
    RUSTFLAGS="-Awarnings" cargo run -q -p aslip_test -- -h
    RUSTFLAGS="-Awarnings" cargo run -q -p aslip_test -- --help
    RUSTFLAGS="-Awarnings" cargo run -q -p aslip_test -- help
    RUSTFLAGS="-Awarnings" cargo run -q -p aslip_test -- -v
    RUSTFLAGS="-Awarnings" cargo run -q -p aslip_test -- --version
    RUSTFLAGS="-Awarnings" cargo run -q -p aslip_test -- version
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test --   no_arg_action
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test --   one_arg 3
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test --   two_arg 3 "this is text."
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test --   arg_9 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_IpAddr      123.123.123.123      
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- two_arg    2 str            
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- no_arg_adction        
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_f32         1      
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_u8          1      
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- renamed         
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- c          
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_CString         str   
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_SocketAddr    "127.0.0.1:8080"    
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_9       1 2 3 4 5 6 7 8 9          
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_i8        1        
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_OsString       1   
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- cmd_doc_example       
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_i128           123   
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_PathBuf      ~/     
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- one_arg        100      
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_SocketAddrV4      "127.0.0.1:8080"
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_char              d
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_i32          3     
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- a2                    
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- no_arg_action         
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- collection_arg     1 str str    
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_u32               6
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- a3452                 
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_NumberInRange     6
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_u16       3        
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- -h                   
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_bool              true
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_u64            3   
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_u128        3      
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_i64         3      
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_usize        3     
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_OnOff         on     
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_String       str      
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_i16            3   
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_f64           4    
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_Ipv6Addr        156:156:0:d2:78:140:140:1  
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_isize         3    
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_Ipv4Addr     123.123.123.123     
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_SocketAddrV6      "[2001:db8::1%3]:8080"

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
    -RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_OnOff "asdf"
    -RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_NumberInRange "99"
    -RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_NumberInRange "1"
    -RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test -- arg_NumberInRange "adsfdsa"
