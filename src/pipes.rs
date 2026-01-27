use std::{
    path::PathBuf,
    process::{Child, Command, Stdio},
};

use crate::{commands, pipes};

pub fn multi_pipe_input(output: &mut Child, command: String, args: Vec<String>) -> Child {
    let input = output.stdout.take().unwrap();
    let output = Command::new(command)
        .args(args)
        .stdin(Stdio::from(input))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    output
}

pub fn single_pipe(
    data_vec: &Vec<String>,
    pipe_pos: usize,
    inbuilt_commands: &Vec<String>,
    path_seperated: &Vec<PathBuf>,
) {
    let command_1 = &data_vec[0];
    let command_2 = &data_vec[pipe_pos + 1];
    let arguments_1 = &data_vec[1..pipe_pos];
    let arguments_2 = &data_vec[pipe_pos + 2..];
    if inbuilt_commands.contains(command_2) {
        if command_2 == "echo" {
            commands::exe_echo(&data_vec);
        }
        if command_2 == "type" {
            commands::exe_type(&data_vec, &path_seperated, &inbuilt_commands);
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
}

pub fn exe_multi_pipe(data_vec: &Vec<String>) {
    let mut new_parsing: Vec<Vec<String>> = Vec::new();
    let mut temp = Vec::new();
    for i in data_vec {
        if i == "|" {
            new_parsing.push(temp.clone());
            temp.clear();
        } else {
            temp.push(i.to_string());
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
        let child = pipes::multi_pipe_input(prev, command, args);
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
        let _ = i.wait();
    }
}
