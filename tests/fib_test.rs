
extern crate bf;
use bf::make_brainfuck_function;
use std::str::from_utf8;
use std::collections::VecDeque;

make_brainfuck_function!(
    "compute_fibonacci",
    ",>>+<<[->[->>+<<]>[-<+>>+<]>[-<+>]<<<]>>.",
    5,
    false
);

#[test]
fn fib() {
    let f: [u8; 13] = [1,1,2,3,5,8,13,21,34,55,89,144,233];
    for n in 0..13 {
        let mut input_data: VecDeque<u8> = VecDeque::from(vec![n]);
        let mut output_data: Vec<u8> = Vec::new();
        compute_fibonacci(&mut input_data, &mut output_data);
    if let Some(&output_char) = output_data.get(0) {
        let fib_number = output_char as u8;
        assert_eq!(fib_number, f[n as usize]);
    } else {
        assert!(false);
    }
    }
}