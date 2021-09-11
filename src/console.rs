use raw_tty::IntoRawMode;
use std::io::{self, stdin, Read, Write};

const BS: u8 = 0x08u8;
const ENTER: u8 = 0x0du8;
const CD: u8 = 0x04u8;

fn print_prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
}

pub fn console() {
    let stdin = stdin().into_raw_mode().unwrap();
    print_prompt();
    for byte in stdin.bytes() {
        match byte.as_ref().unwrap() {
            &0x03u8 => {
                println!("\n\rCtrl + C\r");
                print_prompt();
            }
            &0x7fu8 => {
                print!("{} {}", BS as char, BS as char);
                io::stdout().flush().unwrap();
            }
            &ENTER => {
                println!("\r");
                print_prompt();
            }
            &CD => {
                println!("\n\rCtrl + D \n\rExit!\r");
                break;
            }
            _ => {
                print!("{}", byte.unwrap() as char);
                io::stdout().flush().unwrap();
            }
        }
    }
}
