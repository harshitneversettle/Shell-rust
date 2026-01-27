use std::{os::unix::fs::MetadataExt, path::PathBuf};

pub fn auto_complete(data: &str, no_match: &mut bool) -> String {
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

// this is like , make input as peekabnle , and jbtk there is peekable loop chalao , and if i have to check any i's next element , i just have to do let some(p) = i.next() annd do operations on that

pub fn auto_complete_exe(data: &str, no_match: &mut bool, path_seperated: &Vec<PathBuf>) -> String {
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
