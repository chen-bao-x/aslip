
t1:
    RUSTFLAGS="-Awarnings" cargo run -q -p aslip_test -- 

t2:
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test --   no_arg_action

t3:
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test --   one_arg 3
t4:
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test --   two_arg 3 "this is text."

test:
    just t1  
    just t2
    just t3 
    just t4