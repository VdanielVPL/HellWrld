use std::{io::{ self, Write }, thread, time};

const HELL_WRLD_LEN: usize = 10;
const CHARS: [char; 26] = ascii_uppercase();
const HELL: [char; HELL_WRLD_LEN] = ['H', 'E', 'L', 'L', 'O', 'W', 'O', 'R', 'L', 'D'];

fn main() {
    
    let mut string = String::new();
    let mut x: usize = 0;

    loop {

        for chr in &CHARS {
            thread::sleep(time::Duration::from_millis(100));
            
            print!("\r{}{}", string, chr);
            io::stdout().flush().unwrap();
            if *chr == HELL[x] {
                string.push(*chr);
                x += 1;
                break;
            }
        }

        if x == 5 {
            string.push(' ');
        }
        if x == HELL_WRLD_LEN {
            break;
        }
    }
}

const fn ascii_uppercase() -> [char; 26] {
    let mut chars = ['A'; 26];
    let mut i = 1;

    while i < 26 {
        chars[i] = (chars[i-1] as u8 + 1) as char;
        i += 1;
    }
    
    chars
}
