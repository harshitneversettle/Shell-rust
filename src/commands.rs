use std::{
    env, fs,
    io::{stdout, Write},
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::Command,
};

pub fn exe_command(command: &str, args: &Vec<String>, path_seperated: &Vec<PathBuf>) {
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

pub fn exe_type(
    data_vec: &Vec<String>,
    path_seperated: &Vec<PathBuf>,
    inbuilt_commands: &Vec<String>,
) {
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

pub fn exe_echo(data_vec: &Vec<String>) {
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

pub fn exe_cd(data_vec: &Vec<String>) {
    let new_directory = &data_vec[1];
    let dir_path = Path::new(new_directory);
    if data_vec[1] == "~" {
        let home_dir = env::home_dir().unwrap();
        let _ = env::set_current_dir(home_dir);
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
}

pub fn exe_ls() {
    let present_dir = Path::new(".");
    for i in fs::read_dir(present_dir).unwrap() {
        let file_name = i.as_ref().unwrap().file_name();
        print!("{} ", file_name.display());
        stdout().flush().unwrap();
    }
    println!();
    std::process::exit(1);
}

pub fn exe_pwd() {
    let print_wd = env::current_dir().unwrap();
    println!("{}", print_wd.display());
}

pub fn show_history(history_commands: &Vec<String>, data_vec: &Vec<String>) {
    let mut range: usize = if history_commands.len() > 1 {
        data_vec[1].parse().unwrap()
    } else {
        0
    };
    if range == 0 {
        return;
    }
    for i in &history_commands[range..] {
        println!("{} {}", range + 1, i);
        range += 1;
    }
}
