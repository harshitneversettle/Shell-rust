use std::env::args;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    // print!("$ ") ;
    // io::stdout().flush().unwrap() ;
    // let mut data = String::new() ;
    // io::stdin().read_line(&mut data).unwrap() ;
    // println!("{}: command not found" , data.trim());

    // Repl implementation
    // loop{
    //     // REPL
    //     print!("$ ") ;
    //     io::stdout().flush().unwrap() ;

    //     let mut data = String::new() ;
    //     io::stdin().read_line(&mut data).unwrap() ;
    //     let command = data.trim() ;
    //     println!("{}: command not found" , command) ;
    // }

    // inbuilt exit implementation
    // loop {
    //     print!("$ ") ;
    //     io::stdout().flush().unwrap() ;

    //     let mut data = String::new() ;
    //     io::stdin().read_line(&mut data).unwrap() ;
    //     let command = data.trim() ;

    //     if command == "exit" {break} ;
    //     println!("{}: command not found" , command) ;
    // }

    // echo implementation
    // loop{
    //     print!("$ ") ;
    //     io::stdout().flush().unwrap() ;

    //     let mut data = String::new() ;
    //     io::stdin().read_line(&mut data).unwrap() ;
    //     let command = data.trim() ;

    //     if command == "exit" {
    //         break ;
    //     }

    //     if command.starts_with("echo") {
    //         let mut echo_data = String::new() ;
    //         for i in command.split(" "){
    //             if i == "echo" {continue} ;
    //             echo_data.push_str(i) ;
    //             echo_data.push_str(" ") ;
    //         }
    //         println!("{}" , echo_data.trim())
    //     }else {
    //         println!("{}: command not found" , command) ;
    //     }
    // }

    // The type Builtin
    // loop{
    //     print!("$ ") ;
    //     io::stdout().flush().unwrap() ;

    //     let inbuilt_commands = ["echo", "exit", "type"];
    //     let mut data = String::new() ;
    //     io::stdin().read_line(&mut data).unwrap() ;
    //     let command = data.trim_end() ;
    //     if command == "exit" {break} ;
    //     let commands_vec : Vec<&str> = command.split_whitespace().collect() ;
    //     if commands_vec[0] == "type" {
    //         for i in &commands_vec[1..]{
    //             if inbuilt_commands.contains(i) {
    //                 println!("{} is a shell builtin" , i) ;
    //             }else{
    //                 println!("{} not found" , i) ;
    //             }
    //         }
    //     }else if commands_vec[0] == "echo" {
    //         for i in &commands_vec[1..]{
    //             print!("{} " , i) ;
    //         }
    //         println!();
    //     }else{
    //         println!("{}: not found" , command) ;
    //     }
    // }

    // Path , locate the executable file
    // loop {
    //     print!("$ ");
    //     io::stdout().flush().unwrap();
    //     let path = std::env::var("PATH").unwrap();
    //     let path_seperated: Vec<std::path::PathBuf> = env::split_paths(&path).collect();
    //     // now i have a vector of paths , now command[i] ko attach kro and find that check if it exists or not

    //     let inbuilt_commands = ["echo", "exit", "type"];
    //     let mut data = String::new();
    //     io::stdin().read_line(&mut data).unwrap();
    //     let data_vec: Vec<&str> = data.split_whitespace().collect();

    //     if data_vec.len() == 1 && data_vec[0] == "exit" {
    //         break;
    //     }
    //     if data_vec.len() == 1 && !inbuilt_commands.contains(&data_vec[0]) {
    //         println!("{}: command not found", data_vec[0]);
    //     } else {
    //         if inbuilt_commands.contains(&data_vec[1]) {
    //             println!("{} is a shell builtin", data_vec[1]);
    //             continue;
    //         }

    //         if data_vec[0] == "echo" {
    //             for i in &data_vec[1..] {
    //                 print!("{} ", i);
    //             }
    //             println!();
    //         } else if data_vec[0] == "type" {
    //             for j in &data_vec[1..] {
    //                 let mut flag: bool = false;
    //                 for i in &path_seperated {
    //                     let path_j = Path::new(&j);
    //                     let full_path = i.join(path_j);
    //                     // check if the path generated exists or not
    //                     if full_path.exists() {
    //                         // now check if the path is executable
    //                         let mode = full_path.metadata().unwrap().permissions().mode();
    //                         if (mode & 0o111) != 0 {
    //                             // => the path is executable
    //                             flag = true;
    //                             println!("{} is {}", j, full_path.display());
    //                             break;
    //                         }
    //                     }
    //                 }
    //                 if !flag {
    //                     println!("{}: not found", j);
    //                 }
    //             }
    //         } else {
    //             println!("{}: command not found", data);
    //         }
    //     }
    // }

    // execute a file
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let path = std::env::var("PATH").unwrap();
        let path_seperated: Vec<std::path::PathBuf> = env::split_paths(&path).collect();
        // now i have a vector of paths , now command[i] ko attach kro and find that check if it exists or not

        let inbuilt_commands = ["echo", "exit", "type"];
        let mut data = String::new();
        io::stdin().read_line(&mut data).unwrap();
        let data_vec: Vec<&str> = data.split_whitespace().collect();

        if data_vec.len() == 1 && data_vec[0] == "exit" {
            break;
        }
        if data_vec.len() == 1 && !inbuilt_commands.contains(&data_vec[0]) {
            println!("{}: command not found", data_vec[0]);
        } else if data_vec[0] == "type" {
            if inbuilt_commands.contains(&data_vec[1]) {
                println!("{} is a shell builtin", data_vec[1]);
                continue;
            }
            if data_vec[0] == "echo" {
                for i in &data_vec[1..] {
                    print!("{} ", i);
                }
                println!();
            } else if data_vec[0] == "type" {
                for j in &data_vec[1..] {
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
            } else {
                println!("{}: command not found", data);
            }
        } else {
            // custom_exe_2920 David James David 
            // if the command is this , first one is command name , and the other =s are args 
            let command = &data_vec[0];
            let args = &data_vec[1..];
            //let full_command = i.join(command);
            let result = Command::new(command)
                .args(args)
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        }
    }
}
