use std::fs::{self, OpenOptions};
// use std::io::stdout;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::os::unix::net::UnixDatagram;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::{env, path::Path};
// use crossterm::cursor::MoveTo;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
// use crossterm::execute;
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
                        let mut no_match = false;
                        let mut result = auto_complete(&data, &mut no_match);
                        if no_match {
                            result = auto_complete_exe(&data, &mut no_match, &path_seperated);
                            // println!("{}" , path) ;
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
                    // KeyCode::Char('j') if key.modifiers == KeyModifiers::CONTROL => {
                    //     println!();
                    //     print!("\r");
                    //     io::stdout().flush().unwrap();
                    //     disable_raw_mode().unwrap();
                    //     break;
                    // }
                    KeyCode::Enter => {
                        // there is a crossterm error/bug , the newline(0xA) is mapped with ctrl+J , so i also have to handle ctrl+J ;
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
        let data_vec: Vec<String> = parse_input(&data);
        // parse_input(&data, &mut error_flag, &mut double_redirect, &mut redirect);
        // println!("{:?}" , data_vec) ;
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
        } else if data_vec.len() == 1 && !inbuilt_commands.contains(&data_vec[0]) {
            println!("{}: command not found", data_vec[0]);
            continue;
        } else if redirect {
            exe_redirect(&data_vec, &redirect_pos);
            continue;
        } else if redirect_err {
            exe_redirect_err(&data_vec, &redirect_err_pos);
            continue;
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
                    Err(e) => {
                        println!("{}", e);
                    }
                },
                Err(e) => {
                    println!("{}", e);
                }
            }
        } else if multi_pipe {
            // println!("{:?}" , data_vec) ;
            let mut new_parsing: Vec<Vec<String>> = Vec::new();
            let mut temp = Vec::new();
            for i in data_vec {
                if i == "|" {
                    new_parsing.push(temp.clone());
                    temp.clear();
                } else {
                    temp.push(i);
                }
            }
            if temp.len() != 0 {
                new_parsing.push(temp);
            }
            // println!("{:?}", new_parsing[0][1]);
            let mut children = Vec::new();
            let first = Command::new(&new_parsing[0][0])
                .args(&new_parsing[0][1..])
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();
            children.push(first);
            for i in &new_parsing[1..new_parsing.len() - 1] {
                let prev = children.last_mut().unwrap();
                let command = i[0].clone();
                let args = i[1..].to_vec();
                let child = multi_pipe_input(prev, command, args);
                children.push(child);
            }
            let last = Command::new(&new_parsing[new_parsing.len() - 1][0])
                .args(&new_parsing[new_parsing.len() - 1][1..])
                .stdin(Stdio::from(
                    children.last_mut().unwrap().stdout.take().unwrap(),
                ))
                .spawn()
                .unwrap();
            children.push(last);
            for mut i in children {
                i.wait();
            }
        } else if pipe {
            let command_1 = &data_vec[0];
            let command_2 = &data_vec[pipe_pos + 1];
            let arguments_1 = &data_vec[1..pipe_pos];
            let arguments_2 = &data_vec[pipe_pos + 2..];
            if inbuilt_commands.contains(command_2) {
                if command_2 == "echo" {
                    exe_echo(&data_vec);
                }
                if command_2 == "type" {
                    exe_type(&data_vec, &path_seperated, &inbuilt_commands);
                }
            }
            // Command::new("ls").stdout(Stdio::piped()).output();
            else {
                let mut p1 = Command::new(command_1)
                    .args(arguments_1)
                    .stdout(Stdio::piped())
                    .spawn()
                    .unwrap();
                let output_1 = p1.stdout.take().unwrap(); // take means take the ownership from p1

                let mut p2 = Command::new(command_2)
                    .args(arguments_2)
                    .stdin(Stdio::from(output_1))
                    .spawn()
                    .unwrap();
                p1.wait().unwrap();
                p2.wait().unwrap();
            }

            // println!("{} , {} , {:?} , {:?}" , command_1 , command_2 , arguments_1 , arguments_2) ;
        } else if data_vec[0] == "echo" {
            exe_echo(&data_vec);
        } else if data_vec[0] == "type" {
            exe_type(&data_vec, &path_seperated, &inbuilt_commands);
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
            let args = &data_vec[1..].to_vec();
            exe_command(&command, args, &path_seperated);
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
    //     println!("-----------------------------");f
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
        match i {
            // in match , conditions goes in if , PATTERN if condition => { ... }
            '\\' if !in_single && !in_double => {
                escape = true;
                continue;
            }
            '\\' if in_double => {
                if let Some(&next) = iter.peek() {
                    if next == '\\' || next == '"' {
                        curr_str.push(next);
                        iter.next();
                    } else {
                        curr_str.push(i);
                    }
                }
                continue;
            }
            '\\' if in_single => {
                curr_str.push(i);
                continue;
            }
            _ if escape && !in_double && !in_single => {
                // is saari hi condition hai , start with _
                // for the case \ is outside the quotes
                curr_str.push(i);
                escape = false;
                continue;
            }
            _ if i != ' ' => {
                curr_str.push(i);
            }
            '"' if !in_single => {
                in_double = !in_double;
                continue;
            }
            '\'' if !in_double => {
                in_single = !in_single;
                continue;
            }
            _ if in_single || in_double => {
                curr_str.push(i);
            }
            _ if !curr_str.is_empty() => {
                return_vec.push(curr_str.clone());
                curr_str.clear();
            }
            _ => {}
        }
    }
    if !curr_str.is_empty() {
        return_vec.push(curr_str);
    }
    return_vec
}

fn auto_complete(data: &str, no_match: &mut bool) -> String {
    let mut res = String::new();
    res.push_str(data);
    let res_len = res.len();
    let data2: Vec<&str> = data.split_whitespace().collect();
    if data2.is_empty() {
        *no_match = true;
        return res;
    }
    let to_complete = data2[data2.len() - 1];
    let inventory = vec!["exit", "echo", "type", "pwd"];
    for i in inventory {
        if i.starts_with(to_complete) {
            let pos = to_complete.len();
            let remaning = &i[pos..];
            res.push_str(remaning);
            res.push(' ');
            break;
        }
    }
    if res.len() == res_len {
        *no_match = true;
    } else {
        *no_match = false;
    }

    return res;
}

fn auto_complete_exe(data: &str, no_match: &mut bool, path_seperated: &Vec<PathBuf>) -> String {
    // println!("{:?}" , path_seperated) ;
    // println!("{}" , data) ;
    let mut res = data.to_string();
    let pos = res.len();
    let data2: Vec<&str> = data.split_whitespace().collect();
    if data2.is_empty() {
        *no_match = true;
        return res;
    }
    let to_complete = data2[data2.len() - 1];
    // println!("{}kkkk" , to_complete) ;
    // println!("{:?}" , path_seperated) ;
    // let mut filenames = Vec::new() ;

    for i in path_seperated {
        let read = i.read_dir().unwrap();
        // let mut temp = Vec::new() ;
        for j in read {
            let filename = j.as_ref().unwrap().file_name().into_string().unwrap();
            let mode = j.as_ref().unwrap().metadata().unwrap().mode();
            if mode & 0o111 != 0 && filename.starts_with(&to_complete) {
                res = filename;
                res.push(' ');
                *no_match = false;
                return res.to_string();
            }
            // temp.push(filename);
        }
        // filenames.push(temp);
    }
    // println!("{:?}" , filenames) ;
    if res.len() == pos {
        *no_match = true;
    }
    res.to_string()
}

fn exe_echo(data_vec: &Vec<String>) {
    let mut pos = data_vec.len();
    let mut idx = 0;
    for i in data_vec {
        if i == "echo" {
            pos = idx;
            break;
        }
        idx += 1;
    }
    let effective_data_vec = &data_vec[pos..];
    let mut temp_str = String::new();
    for i in &effective_data_vec[1..] {
        temp_str.push_str(i);
        temp_str.push(' ');
    }
    print!("{}", temp_str.trim_end());
    println!();
}

fn exe_command(command: &str, args: &Vec<String>, path_seperated: &Vec<PathBuf>) {
    let mut found = false;
    for i in path_seperated {
        let full_cmd = i.join(command);
        if full_cmd.exists() {
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

fn exe_type(data_vec: &Vec<String>, path_seperated: &Vec<PathBuf>, inbuilt_commands: &Vec<String>) {
    let mut pos = data_vec.len();
    let mut idx = 0;
    for i in data_vec {
        if i == "type" {
            pos = idx;
            break;
        }
        idx += 1;
    }
    let effective_data_vec = data_vec[pos..].to_vec();
    for j in &effective_data_vec[1..] {
        if inbuilt_commands.contains(j) {
            println!("{} is a shell builtin", j);
            continue;
        }
        let mut flag: bool = false;
        for i in path_seperated {
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
}

fn exe_redirect(data_vec: &Vec<String>, redirect_pos: &usize) {
    let pos = *redirect_pos as usize;
    let command = &data_vec[0];
    let args = &data_vec[1..pos];
    let filename = &data_vec[pos + 1];
    let file = match OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(filename)
    {
        Result::Ok(f) => f, // both the arms of match should handle the same type , like file file , or none none , etc etc
        Err(e) => {
            println!("{}", e);
            std::process::exit(1); // its type is !
        }
    };
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

fn exe_redirect_err(data_vec: &Vec<String>, redirect_err_pos: &usize) {
    let pos = *redirect_err_pos as usize;
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
            std::process::exit(1);
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
}

// this is like , make input as peekabnle , and jbtk there is peekable loop chalao , and if i have to check any i's next element , i just have to do let some(p) = i.next() annd do operations on that

fn multi_pipe_input(output: &mut Child, command: String, args: Vec<String>) -> Child {
    let input = output.stdout.take().unwrap();
    let output = Command::new(command)
        .args(args)
        .stdin(Stdio::from(input))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    output
}
