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
            '"' if !in_single => {
                in_double = !in_double;
                continue;
            }
            '\'' if !in_double => {
                in_single = !in_single;
                continue;
            }
            _ if i != ' ' => {
                curr_str.push(i);
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
