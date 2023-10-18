use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("opsay (c) 2023 kkzero");
        println!("USAGE: opsay [options] <text>");
        println!("OPTIONAL ARGS:");
        println!("-l: Lessen output to just name and acronym");
        println!("-n <name>: Change org name to <name>");
    }
    else {
        let mut backronym_full: Vec<String> = Vec::new();
        let mut name: &str = "opsay";
        let mut ops_done: bool = false;
        let mut flag_name: bool = false;
        let mut lessen: bool = false;

        // Iterate through args and check what's an arg and what's string data
        for i in &args[1..] {
            // If -n, the name flag, get the next text and change the "X mission" text to say that as X
            if flag_name {
                name = i.as_str();
                flag_name = false;
                continue;
            }
            if !ops_done && i.eq("-n") {
                flag_name = true;
                continue;
            }
            // If -l, the lessen output flag, just name and acronym will be displayed
            if !ops_done && i.eq("-l") {
                lessen = true;
                continue;
            }
            // Append any loose non-alphabetics to the last word
            if ops_done && !i.contains(char::is_alphabetic) {
                if backronym_full.len() > 0 {
                    let last: String = backronym_full.pop().unwrap();
                    backronym_full.push(last + " " + i);
                }
                ops_done = true;
                continue;
            }
            // Presence of whitespace means i was entered in quotes and needs parsing
            if i.contains(char::is_whitespace) {
                ops_done = true;
                let quoted: Vec<&str> = i.as_str().split_whitespace().collect();
                for q in quoted {
                    // Check for loose non-alphabetics here too
                    if !q.contains(char::is_alphabetic) {
                        let last: String = backronym_full.pop().unwrap();
                        backronym_full.push(last + " " + q);
                        continue;
                    }
                    backronym_full.push(q.to_string());
                }
                continue;
            }
            ops_done = true;
            backronym_full.push(i.to_string());
        }
        let mut backronym: String = String::new();
        for i in &backronym_full {
            // Grab the first character in the string that's alphabetic
            let mut inchar: char = ' ';
            for c in i.chars().rev() {
                if c.is_alphabetic() {
                    inchar = c;
                }
            }
            if inchar.is_alphabetic() {
                backronym.push(inchar);
                backronym.push('.');
            }
        }

        // Account for the input having no alphabetic characters
        if backronym.len() < 1 {
            backronym.push_str("[no alphabetics]");
        }

        // Now start printing out the acronym and its contents
        if lessen {
            println!("{}\n{}\n", name, backronym);
        }
        else {
            println!("now loading\n{} mission\noperation:\n{}\n", name, backronym);
        }
        
        for i in &backronym_full {
            // Output each word in the acronym
            println!{"{}", i};
        }
    }
}
