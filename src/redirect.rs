use std::{
    fs::OpenOptions,
    process::{Command, Stdio},
};

pub fn exe_redirect(data_vec: &Vec<String>, redirect_pos: &usize) {
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

pub fn exe_redirect_err(data_vec: &Vec<String>, redirect_err_pos: &usize) {
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

pub fn double_redirect_err(double_redirect_err_pos: usize, data_vec: &Vec<String>) {
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
            Err(e) => {
                println!("{}", e);
            }
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}

pub fn double_redirect(data_vec: &Vec<String>, double_redirect_pos: usize) {
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
            std::process::exit(1);
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
}
