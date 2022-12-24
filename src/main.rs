use std::{env, fs, collections};

fn main() {

    let reserved_cmds = vec!["save", "delete"];

    let cmd = env::args().nth(1).unwrap();

    if reserved_cmds.iter().all(|&c| c != cmd){
        let default = "None";

        // path should use $HOME/.goto/.dict
        let directory_file = fs::read_to_string("/Users/kimberlyluna/.goto-dic").unwrap();
        
        let mut available_cmds = collections::HashMap::new();


        for line in directory_file.lines() {
            let parts: Vec<&str> = line.split("=").collect();

            let alias = parts[0];
            let path = parts[1];

            available_cmds.insert(alias, path);
        }

        let goto_path  = available_cmds.get(&*cmd).unwrap_or(&default);

        if goto_path == &default {
            println!("alias not found!");
            return
        }
        
        println!("{}", goto_path);
        return
    }

    match cmd.as_str() {
        "save" => println!("Not implemented yet"), // goto save (alias) (absolute-path)
        "delete"  => println!("Not implemented yet"),  // goto delete (alias) ---> Are you sure you want to delete (alias)? (Y/N)
        _ => println!("unrecogized command")
    }
   
}