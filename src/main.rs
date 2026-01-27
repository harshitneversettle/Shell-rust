// use std::io::stdout;
use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::PathBuf;
// use crossterm::cursor::MoveTo;
// use crossterm::execute;
use crossterm::terminal::enable_raw_mode;
mod auto_complete;
mod commands;
mod input;
mod parser;
mod pipes;
mod redirect;
fn main() {
    let mut history_commands: Vec<String> = Vec::new();
    let path = std::env::var("PATH").unwrap();
    let path_seperated: Vec<PathBuf> = env::split_paths(&path).collect();
    let inbuilt_commands = vec![
        "echo".to_string(),
        "exit".to_string(),
        "type".to_string(),
        "pwd".to_string(),
        "cd".to_string(),
        "history".to_string(),
    ];
    // now i have a vector of paths , now command[i] ko attach kro and find that check if it exists or not
    loop {
        enable_raw_mode().unwrap();
        print!("$ ");
        io::stdout().flush().unwrap();
        // let symbol1 = String::from(">");
        let mut data = String::new();
        let mut his_idx: u32 = 0;
        // io::stdin().read_line(&mut data).unwrap();
        // taking input
        input::take_input(&history_commands, &mut data, &path_seperated, &mut his_idx);
        // let error_flag = false;
        let mut redirect = false;
        let mut redirect_pos = 0;
        let mut double_redirect = false;
        let mut double_redirect_pos = 0;
        let mut redirect_err = false;
        let mut redirect_err_pos = 0;
        let mut double_redirect_err = false;
        let mut double_redirect_err_pos = 0;
        let mut pipe = false;
        let mut pipe_pos = 0;
        let mut multi_pipe = false;
        history_commands.push(data);
        let data_vec: Vec<String> = parser::parse_input(&history_commands.last().unwrap());
        // parse_input(&data, &mut error_flag, &mut double_redirect, &mut redirect);
        // println!("{:?}", data_vec);
        let mut count = 0;
        for i in &data_vec {
            if i == "|" {
                count += 1;
            }
        }
        if count > 1 {
            multi_pipe = true;
        }
        for (idx, i) in data_vec.iter().enumerate() {
            match i.as_str() {
                ">>" | "1>>" => {
                    double_redirect = true;
                    double_redirect_pos = idx;
                }
                ">" | "1>" => {
                    redirect = true;
                    redirect_pos = idx;
                }
                "2>" => {
                    redirect_err = true;
                    redirect_err_pos = idx;
                }
                "2>>" => {
                    double_redirect_err = true;
                    double_redirect_err_pos = idx;
                }
                "|" => {
                    pipe = true;
                    pipe_pos = idx;
                }
                _ => {}
            }
        }
        if data_vec.is_empty() {
            continue;
        } else if data_vec.len() == 1 && data_vec[0] == "exit" {
            break;
        } else if data_vec.len() == 1
            && !inbuilt_commands.contains(&data_vec[0])
            && data_vec[0] != "ls"
        {
            println!("{}: command not found", data_vec[0]);
            continue;
        } else if data_vec.len() <= 2 && data_vec[0] == "history" {
            commands::show_history(&history_commands, &data_vec);
        } else if redirect {
            redirect::exe_redirect(&data_vec, &redirect_pos);
            continue;
        } else if redirect_err {
            redirect::exe_redirect_err(&data_vec, &redirect_err_pos);
            continue;
        } else if double_redirect {
            redirect::double_redirect(&data_vec, double_redirect_pos);
        } else if double_redirect_err {
            redirect::double_redirect_err(double_redirect_err_pos, &data_vec);
        } else if multi_pipe {
            pipes::exe_multi_pipe(&data_vec);
        } else if pipe {
            pipes::single_pipe(&data_vec, pipe_pos, &inbuilt_commands, &path_seperated);
        } else if data_vec[0] == "ls" {
            commands::exe_ls();
        } else if data_vec[0] == "echo" {
            commands::exe_echo(&data_vec);
        } else if data_vec[0] == "type" {
            commands::exe_type(&data_vec, &path_seperated, &inbuilt_commands);
        } else if data_vec[0] == "pwd" {
            commands::exe_pwd();
        } else if data_vec[0] == "cd" {
            commands::exe_cd(&data_vec);
        } else {
            // if the command is this , first one is command name , and the other =s are args
            commands::exe_command(&data_vec[0], &data_vec[1..].to_vec(), &path_seperated);
        }
    }
}
