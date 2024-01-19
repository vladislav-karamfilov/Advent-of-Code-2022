use std::collections::HashMap;

fn main() {
    // solve_puzzle1();
    solve_puzzle2();
}

#[allow(dead_code)]
fn solve_puzzle1() {
    let mut directories_by_path: HashMap<String, Directory> = HashMap::new();
    let mut current_directory_path = String::new();

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let line = line.trim();
        if line.is_empty() {
            break;
        }

        if line.starts_with('$') {
            current_directory_path = handle_command_line(line, current_directory_path);
        } else {
            handle_file_entry_line(line, &current_directory_path, &mut directories_by_path);
        }
    }

    let mut sum_of_target_directories = 0u64;

    for (_, directory) in directories_by_path.iter() {
        let directory_size = calculate_directory_size(directory, &directories_by_path, false);
        if directory_size <= 100_000u64 {
            sum_of_target_directories += directory_size;
        }
    }

    println!("{sum_of_target_directories}");
}

#[allow(dead_code)]
fn solve_puzzle2() {
    let mut directories_by_path: HashMap<String, Directory> = HashMap::new();
    let mut current_directory_path = String::new();

    loop {
        let mut line = String::new();

        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let line = line.trim();
        if line.is_empty() {
            break;
        }

        if line.starts_with('$') {
            current_directory_path = handle_command_line(line, current_directory_path);
        } else {
            handle_file_entry_line(line, &current_directory_path, &mut directories_by_path);
        }
    }

    let mut used_space = 0;
    for (_, directory) in directories_by_path
        .iter()
        .filter(|(path, _)| *path == "/" || !path[1..].contains('/'))
    {
        let size = calculate_directory_size(directory, &directories_by_path, directory.path == "/");
        used_space += size;
        println!("{} - {}", directory.path, size);
    }

    println!("used: {used_space}");

    let unused_space = 70_000_000 - used_space;
    println!("unused: {unused_space}");

    let space_left_to_free = 30_000_000 - unused_space;
    let mut min_directory_size_to_free = u64::MAX;

    for (_, directory) in directories_by_path.iter() {
        let directory_size = calculate_directory_size(directory, &directories_by_path, false);
        if directory_size >= space_left_to_free && min_directory_size_to_free > directory_size {
            min_directory_size_to_free = directory_size;
        }
    }

    println!("{min_directory_size_to_free}");
}

fn calculate_directory_size(
    directory: &Directory,
    directories_by_path: &HashMap<String, Directory>,
    skip_subdirectories: bool,
) -> u64 {
    let files_size: u64 = directory.files.iter().map(|f| f.size).sum();
    if skip_subdirectories {
        return files_size;
    }

    let mut subdirectories_size = 0;
    for subdirectory_name in &directory.subdirectories {
        let separator = if directory.path == "/" { "" } else { "/" };
        let subdirectory_path = format!("{}{separator}{subdirectory_name}", directory.path);

        match &directories_by_path.get(&subdirectory_path) {
            Some(d) => {
                subdirectories_size += calculate_directory_size(d, directories_by_path, false)
            }
            None => continue,
        }
    }

    files_size + subdirectories_size
}

fn handle_command_line(line: &str, current_directory_path: String) -> String {
    if line == "$ ls" {
        return current_directory_path;
    }

    if line == "$ cd .." {
        match current_directory_path.rfind('/') {
            Some(0) => return "/".to_string(),
            Some(i) => return current_directory_path[..i].to_string(),
            None => return current_directory_path,
        }
    }

    let directory = &line[5..];
    let separator = if current_directory_path.len() < 2 {
        ""
    } else {
        "/"
    };

    format!("{current_directory_path}{separator}{directory}")
}

fn handle_file_entry_line(
    line: &str,
    current_directory_path: &str,
    directories_by_path: &mut HashMap<String, Directory>,
) {
    if !directories_by_path.contains_key(current_directory_path) {
        directories_by_path.insert(
            current_directory_path.to_string(),
            Directory {
                path: current_directory_path.to_string(),
                files: Vec::new(),
                subdirectories: Vec::new(),
            },
        );
    }

    let directory = directories_by_path.get_mut(current_directory_path).unwrap();

    let mut directory_entry_splitter = line.split(' ');
    let entry_first_part = directory_entry_splitter.next().unwrap();
    if entry_first_part == "dir" {
        directory
            .subdirectories
            .push(directory_entry_splitter.next().unwrap().to_string());
    } else {
        directory.files.push(File {
            name: directory_entry_splitter.next().unwrap().to_string(),
            size: entry_first_part.parse().unwrap(),
        });
    }
}

#[allow(dead_code)]
struct File {
    name: String,
    size: u64,
}

struct Directory {
    path: String,
    files: Vec<File>,
    subdirectories: Vec<String>,
}
