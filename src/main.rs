use std::fs::{self, OpenOptions};
#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::process::{Command, Stdio};
use std::{env, path::Path};

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

fn main() {
    let path = std::env::var("PATH").unwrap();
    let path_seperated: Vec<std::path::PathBuf> = env::split_paths(&path).collect();

    // now i have a vector of paths , now command[i] ko attach kro and find that check if it exists or not
    loop {
        enable_raw_mode().unwrap();
        print!("$ ");
        io::stdout().flush().unwrap();

        let inbuilt_commands = vec![
            "echo".to_string(),
            "exit".to_string(),
            "type".to_string(),
            "pwd".to_string(),
            "cd".to_string(),
        ];
        // let symbol1 = String::from(">");
        let mut data = String::new();
        // io::stdin().read_line(&mut data).unwrap();
        // taking input
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
                    KeyCode::Tab => {
                        let result = auto_complete(&data);
                        data.clear();
                        print!("\r");
                        data.push_str(&result);
                        print!("$ {}", data);
                        io::stdout().flush().unwrap();
                        continue;
                    }
                    KeyCode::Enter => {
                        println!();
                        print!("\r");
                        disable_raw_mode().unwrap();
                        io::stdout().flush().unwrap();
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

        let error_flag = false;
        let mut redirect = false;
        let mut redirect_pos = 0;
        let mut double_redirect = false;
        let mut double_redirect_pos = 0;
        let mut redirect_err = false;
        let mut redirect_err_pos = 0;
        let mut double_redirect_err = false;
        let mut double_redirect_err_pos = 0;
        let data_vec: Vec<String> = parse_input(&data);
        // parse_input(&data, &mut error_flag, &mut double_redirect, &mut redirect);
        // print!("{}" , double_redirect) ;
        let mut idx = 0;
        for i in &data_vec {
            if i == ">>" || i == "1>>" {
                double_redirect = true;
                double_redirect_pos = idx;
            } else if i == ">" || i == "1>" {
                redirect = true;
                redirect_pos = idx;
            } else if i == "2>" {
                redirect_err = true;
                redirect_err_pos = idx;
            } else if i == "2>>" {
                double_redirect_err = true;
                double_redirect_err_pos = idx;
            }
            idx += 1;
        }
        if data_vec.is_empty() {
            continue;
        } else if data_vec.len() == 1 && data_vec[0] == "exit" {
            break;
        } else if data_vec.len() == 1 && !inbuilt_commands.contains(&data_vec[0]) {
            println!("{}: command not found", data_vec[0]);
            continue;
        } else if redirect {
            let pos = redirect_pos as usize;
            let command = &data_vec[0];
            let args = &data_vec[1..pos];
            let filename = &data_vec[pos + 1];
            let file = match OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(filename)
            {
                Result::Ok(f) => f,
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            };
            if error_flag {
                match Command::new(&command)
                    .args(args)
                    .stderr(Stdio::from(file))
                    .spawn()
                {
                    Result::Ok(mut child) => match child.wait() {
                        Result::Ok(_) => {}
                        Err(_) => {}
                    },
                    Err(_) => {}
                }
            } else {
                match Command::new(&command)
                    .args(args)
                    .stdout(Stdio::from(file))
                    .spawn()
                {
                    Result::Ok(mut child) => match child.wait() {
                        Result::Ok(_) => {}
                        Err(_) => {}
                    },
                    Err(_) => {}
                }
            }
            continue;
        } else if redirect_err {
            let pos = redirect_err_pos as usize;
            let command = &data_vec[0];
            let args = &data_vec[1..pos];
            let filename = &data_vec[pos + 1];
            let file = match OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(filename)
            {
                Result::Ok(f) => f,
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            };

            match Command::new(command)
                .args(args)
                .stderr(Stdio::from(file))
                .spawn()
            {
                Result::Ok(mut child) => match child.wait() {
                    Result::Ok(_) => {}
                    Err(_) => {}
                },
                Err(_) => {}
            }
        } else if double_redirect {
            let pos = double_redirect_pos as usize;
            let command = &data_vec[0];
            let filename = &data_vec[pos + 1];
            let args = &data_vec[1..pos];
            // print!("{:?} , {:?} , {:?} , {}" , filename , args , command , pos) ;
            //let mut idx = 1;
            // println!("{}" , filename) ;
            let file = match OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .append(true)
                .open(filename)
            {
                Result::Ok(f) => f,
                Err(e) => {
                    eprintln!("{}: {}", filename, e);
                    continue;
                }
            };
            match Command::new(command)
                .args(args)
                .stdout(Stdio::from(file))
                .spawn()
            {
                Result::Ok(mut child) => match child.wait() {
                    Result::Ok(_) => {}
                    Err(_) => {}
                },
                Err(_) => {}
            }
        } else if double_redirect_err {
            let pos = double_redirect_err_pos as usize;
            let command = &data_vec[0];
            let args = &data_vec[1..pos];
            let filename = &data_vec[pos + 1];
            let file = match OpenOptions::new()
                .create(true)
                .read(true)
                .write(true)
                .append(true)
                .open(filename)
            {
                Result::Ok(f) => f,
                Err(e) => {
                    print!("{}", e);
                    continue;
                }
            };
            match Command::new(command)
                .args(args)
                .stderr(Stdio::from(file))
                .spawn()
            {
                Result::Ok(mut child) => match child.wait() {
                    Result::Ok(_) => {}
                    Err(e) => {}
                },
                Err(e) => {}
            }
        } else if data_vec[0] == "echo" {
            let mut temp_str = String::new();
            for i in &data_vec[1..] {
                temp_str.push_str(i);
                temp_str.push(' ');
            }
            print!("{}", temp_str.trim_end());
            println!();
        } else if data_vec[0] == "type" {
            for j in &data_vec[1..] {
                if inbuilt_commands.contains(j) {
                    println!("{} is a shell builtin", j);
                    continue;
                }
                let mut flag: bool = false;
                for i in &path_seperated {
                    let path_j = Path::new(&j);
                    let full_path = i.join(path_j);
                    // check if the path generated exists or not
                    if full_path.exists() {
                        // now check if the path is executable
                        let mode = full_path.metadata().unwrap().permissions().mode();
                        if (mode & 0o111) != 0 {
                            // => the path is executable
                            flag = true;
                            println!("{} is {}", j, full_path.display());
                            break;
                        }
                    }
                }
                if !flag {
                    println!("{}: not found", j);
                }
            }
        } else if data_vec[0] == "pwd" {
            let print_wd = env::current_dir().unwrap();
            println!("{}", print_wd.display());
        } else if data_vec[0] == "cd" {
            let new_directory = &data_vec[1];
            let dir_path = Path::new(new_directory);
            if data_vec[1] == "~" {
                let home_dir = env::home_dir().unwrap();
                let _ = env::set_current_dir(home_dir);
                continue;
            }
            if fs::exists(dir_path).unwrap() {
                let _ = env::set_current_dir(new_directory);
            } else {
                // cd: /does_not_exist: No such file or directory
                println!(
                    "{}: {}: No such file or directory",
                    data_vec[0],
                    dir_path.display()
                );
            }
        } else {
            // if the command is this , first one is command name , and the other =s are args
            // println!("{:?}" , data_vec) ;
            let command = &data_vec[0];
            let args = &data_vec[1..];
            let mut found = false;
            for i in &path_seperated{
                let full_cmd = i.join(command) ;
                if full_cmd.exists(){
                    Command::new(&full_cmd)
                .args(args)
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
                found = true;
                break;
                }
                 
            }
            if !found {
                println!("{}: command not found", command);
            }
        }
    }
    // let tests = vec![
    //     "-1 nonexistent",
    //     "-1 nonexistent > out.txt",
    //     "-1 nonexistent 2> err.txt",
    //     "-1 nonexistent > out.txt 2> err.txt",
    //     "-1 nonexistent 2> err.txt > out.txt",
    //     r#"echo "2> not redirect""#,
    //     r#"echo ">""#,
    // ];

    // for input in tests {
    //     let mut error_flag = false;
    //     let result = parse_input(input, &mut error_flag);

    //     println!("INPUT  : {:?}", input);
    //     println!("TOKENS : {:?}", result);
    //     println!("ERROR? : {}", error_flag);
    //     println!("-----------------------------");
    // }
}

pub fn parse_input(
    input: &str,
    // error_flag: &mut bool,
    // double_redirect: &mut bool,
    // redirect: &mut bool,
) -> Vec<String> {
    let mut return_vec: Vec<String> = Vec::new();
    let mut in_single: bool = false;
    let mut in_double: bool = false;
    let mut escape: bool = false;
    let mut curr_str = String::new();
    let input = input.trim_end_matches(|c| c == '\n' || c == '\r');

    let mut iter = input.chars().peekable();
    //print!("{}" , input) ;
    while let Some(i) = iter.next() {
        if i == '\\' && !in_single && !in_double {
            // this is the case ki slash occurs but with no quotes
            escape = true;
            continue;
        }
        if i == '\\' && in_double {
            if let Some(&next) = iter.peek() {
                if next == '\\' || next == '"' {
                    curr_str.push(next);
                    iter.next();
                } else {
                    curr_str.push(i);
                }
            }
            continue;
        } else if i == '\\' && in_single {
            curr_str.push(i);
            continue;
        } else if escape && !in_double && !in_single {
            // for the case \ is outside the quotes
            curr_str.push(i);
            escape = false;
            continue;
        } else if i == '"' && !in_single {
            in_double = !in_double;
            continue;
        } else if i == '\'' && !in_double {
            in_single = !in_single;
            continue;
        } else if in_single || in_double {
            curr_str.push(i);
        } else if i != ' ' {
            curr_str.push(i);
        } else if !curr_str.is_empty() {
            return_vec.push(curr_str.clone());
            curr_str.clear();
        }
    }
    if !curr_str.is_empty() {
        return_vec.push(curr_str);
    }
    return_vec
}

fn auto_complete(data: &str) -> String {
    let mut res = String::new();
    res.push_str(data);
    let inventory = vec!["exit", "echo"];
    for i in inventory {
        if i.starts_with(data) {
            let pos = data.len();
            let remaning = &i[pos..];
            res.push_str(remaning);
            res.push(' ');
            break;
        }
    }

    return res;
}
