#[allow(unused_imports)]
use std::io::{self, Write};

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
    loop{
        print!("$ ") ;
        io::stdout().flush().unwrap() ;

        let inbuilt_commands = ["echo", "exit", "type"];
        let mut data = String::new() ;
        io::stdin().read_line(&mut data).unwrap() ;
        let command = data.trim_end() ;
        if command == "exit" {break} ;
        let commands_vec : Vec<&str> = command.split_whitespace().collect() ;
        if commands_vec[0] == "type" {
            for i in &commands_vec[1..]{
                if inbuilt_commands.contains(i) {
                    println!("{} is a shell builtin" , i) ;
                }else{
                    println!("{} not found" , i) ;
                }
            }
        }else if commands_vec[0] == "echo" {
            for i in &commands_vec[1..]{
                print!("{} " , i) ;
            }
            println!();
        }else{
            println!("{}: not found" , command) ;
        }
    }
    

    
   
}
