use std::fs::read_dir;


use accounters_lib::data::Database;

pub fn load_database(dir_path: &str) -> Option<(String, Database)> {
    let paths = find_databases(dir_path).unwrap();
    loop {
        let n_lines = termsize::get().unwrap().rows as usize;
        println!("Available databases in {dir_path}:");
        for (index, name) in paths.iter().enumerate() {
            println!("\t{}) {}", index+1, name)
        }
        println!("{}", "\n".repeat(n_lines - 4 - paths.len()));
        println!("Select by index or name, or press q to quit:");
        let mut input = String::new();

        std::io::stdin().read_line(&mut input).unwrap();
        let trimmed_input = input.trim();
        
        if let Ok(index) = trimmed_input.parse::<usize>() {
            let Some(name) = paths.get(index - 1) else {
                println!("Index out of bounds");
                continue
            };
            println!("{name} loaded");
            return Some((
                String::from(name),
                Database::read_from_file(&format!("{dir_path}/{name}.json")).unwrap()
            ));
        }

        if paths.iter().any(|x| x == trimmed_input) {
            println!("{trimmed_input} loaded");
            return Some((
                String::from(trimmed_input),
                Database::read_from_file(&format!("{dir_path}/{trimmed_input}.json")).unwrap()
            ));
        }

        if trimmed_input == "q" {
            return None;
        }

        println!("\nThe file IS NOT THERE!");
    }
}


fn find_databases(path: &str) -> Option<Vec<String>> {
    Some(
        read_dir(path).ok()?.filter_map(|file| {
            let path_buf = file.ok()?.path();
            let path = path_buf.as_path();

            if path.extension()? != "json" {
                return None
            }
            Some(path.file_stem()?.to_str()?.to_owned())
        }).collect()
    )
}
