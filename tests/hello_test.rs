
extern crate bf;
use bf::make_brainfuck_function;
use std::str::from_utf8;
use std::collections::VecDeque;

make_brainfuck_function!("hello", "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.", 200, false);



#[test]
fn hello_test() {

    let mut a : Vec<u8> = Vec::new();
    let mut b : VecDeque<u8> = VecDeque::new();
    hello(&mut b, &mut a);
    let s = from_utf8(&a).unwrap();
    assert_eq!(s, "Hello World!\n");
}

