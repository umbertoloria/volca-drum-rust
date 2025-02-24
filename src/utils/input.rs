use std::io::{stdin, stdout, Write};

pub fn get_console_int_input(message: &str, min_incl: u8, max_incl: u8) -> u8 {
    let mut int_input: u8 = 0;
    while {
        print!("{}", message);
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        int_input = input.trim().parse::<u8>().unwrap();

        int_input < min_incl || int_input > max_incl
    } {}
    int_input
}
