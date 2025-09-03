
t1:
    RUSTFLAGS="-Awarnings" cargo run -q -p aslip_test -- 

t2:
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test --   no_arg_action

t3:
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test --   one_arg 3

t4:
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test --   two_arg 3 "this is text."

t5:
    RUSTFLAGS="-Awarnings" cargo run  -q -p aslip_test --   arg_9 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18

test:
    just t1  
    just t2
    just t3 
    just t4
    just t5