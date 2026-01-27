use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::disable_raw_mode,
};
use std::{
    io::{self, Write},
    path::PathBuf,
};

use crate::auto_complete;

pub fn take_input(
    history_commands: &Vec<String>,
    data: &mut String,
    path_seperated: &Vec<PathBuf>,
    his_idx: &mut u32,
) {
    let his_len = history_commands.len() as u32;
    loop {
        if let Event::Key(key) = event::read().unwrap() {
            if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                crossterm::terminal::disable_raw_mode().unwrap();
                std::process::exit(0); // ends the whole process immediately
            }
            match key.code {
                KeyCode::Char(c) => {
                    data.push(c);
                    print!("{}", c);
                    io::stdout().flush().unwrap();
                }
                KeyCode::Up => {
                    // if his_idx == 0 {continue;}
                    *his_idx += 1;
                    if *his_idx <= his_len {
                        let req_idx = (his_len - *his_idx) as usize;
                        if req_idx <= history_commands.len() - 1 {
                            print!("{} ", history_commands[req_idx]);
                            data.clear();
                            data.push_str(&history_commands[req_idx]);
                            io::stdout().flush().unwrap();
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }
                KeyCode::Down => {
                    if *his_idx == 0 {
                        continue;
                    }
                    *his_idx -= 1;
                    if *his_idx <= his_len {
                        let req_idx = (his_len - *his_idx) as usize;
                        if req_idx <= history_commands.len() - 1 {
                            print!("{} ", history_commands[req_idx]);
                            io::stdout().flush().unwrap();
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }
                KeyCode::Tab => {
                    let mut no_match = false;
                    let mut result = auto_complete::auto_complete(&data, &mut no_match);
                    if no_match {
                        result =
                            auto_complete::auto_complete_exe(&data, &mut no_match, &path_seperated);
                    }
                    if no_match {
                        print!("\x07");
                        io::stdout().flush().unwrap();
                    } else {
                        data.clear();
                        print!("\r");
                        data.push_str(&result);
                        print!("$ {}", data);
                        io::stdout().flush().unwrap();
                        continue;
                    }
                }
                KeyCode::Enter => {
                    // there is a crossterm error/bug , the newline(0xA) is mapped with ctrl+J , so i also have to handle ctrl+J ;
                    // his_idx = 0;
                    println!();
                    print!("\r");
                    io::stdout().flush().unwrap();
                    disable_raw_mode().unwrap();
                    break;
                }
                KeyCode::Backspace => {
                    if !data.is_empty() {
                        data.pop();
                        print!("\u{0008} \u{0008}"); // this only moves the cursor/pointer to the left
                        io::stdout().flush().unwrap();
                    }
                }
                _ => {}
            }
        }
    }
}
