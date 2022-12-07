use std::collections::HashMap;
use twentytwo::{print_solution, read_from_stdin};

pub type NodeName = String;
pub type Path = Vec<String>;
pub type Directory = Vec<Node>;

// My filesystem consists of paths pointing to a directory, which is a list of nodes
pub type Filesystem = HashMap<Path, Directory>;

#[derive(Clone)]
pub struct Node(pub NodeName, pub NodeType);

#[derive(Clone)]
pub enum NodeType {
    Directory,
    File(u64),
}

fn main() {
    let input = read_from_stdin();

    print_solution(
        7,
        1,
        "How many characters need to be processed before the first start-of-packet marker is detected?",
        format!("{}", sum_of_directories(&input))
    );

    print_solution(
        7,
        2,
        "How many characters need to be processed before the first start-of-packet marker is detected?",
        format!("{}", size_of_smallest_directory_to_delete(&input))
    );
}

// D7P1
fn sum_of_directories(input: &str) -> u64 {
    let commands = parse_commands(input).expect("parse commands");
    let filesystem = build_filesystem(commands);

    list_of_directory_sizes(&filesystem)
        .iter()
        .filter(|&size| size <= &100_000)
        .sum()
}

// D7P2
fn size_of_smallest_directory_to_delete(input: &str) -> u64 {
    let commands = parse_commands(input).expect("parse commands");
    let filesystem = build_filesystem(commands);

    let free_space = 70_000_000 - calculate_directory_size(&vec!["/".to_string()], &filesystem);

    let mut directory_sizes = list_of_directory_sizes(&filesystem);
    directory_sizes.sort();

    *directory_sizes
        .iter()
        .find(|size| *size + free_space >= 30_000_000)
        .expect("find dir")
}

fn list_of_directory_sizes(filesystem: &Filesystem) -> Vec<u64> {
    filesystem
        .keys()
        .collect::<Vec<&Vec<String>>>()
        .iter()
        .map(|&key| (key, calculate_directory_size(key, filesystem)))
        .map(|t| t.1)
        .collect()
}

fn calculate_directory_size(path: &Vec<String>, filesystem: &Filesystem) -> u64 {
    let contents = filesystem.get(path).expect("get directory list");

    contents
        .iter()
        .map(|node| match node.1 {
            NodeType::Directory => {
                let mut dir_path = path.clone();
                dir_path.push(node.0.clone());

                calculate_directory_size(&dir_path, filesystem)
            }
            NodeType::File(size) => size,
        })
        .sum()
}

// Parsing -------->

#[derive(Debug)]
pub enum CommandType {
    Cd(String),
    CdParentDir,
    Ls(Vec<LsLine>),
}

#[derive(Debug)]
pub enum LsLine {
    Directory(String),
    File(String, u64),
}

fn parse_commands(input: &str) -> Result<Vec<CommandType>, ()> {
    input
        .trim()
        .split("$ ")
        .filter(|chunk| !chunk.is_empty())
        .map(|chunk| chunk.trim())
        .map(chunk_to_command)
        .collect()
}

fn chunk_to_command(chunk: &str) -> Result<CommandType, ()> {
    if chunk.starts_with("ls") {
        let ls_output = chunk
            .lines()
            .skip(1)
            .map(parse_ls_output_line)
            .collect::<Result<Vec<LsLine>, ()>>()
            .expect("parse ls output");

        Ok(CommandType::Ls(ls_output))
    } else if chunk.starts_with("cd ..") {
        Ok(CommandType::CdParentDir)
    } else if let Some(chunk_without_prefix) = chunk.strip_prefix("cd ") {
        Ok(CommandType::Cd(chunk_without_prefix.to_string()))
    } else {
        Err(())
    }
}

fn parse_ls_output_line(line: &str) -> Result<LsLine, ()> {
    let white_space_pos = line
        .chars()
        .enumerate()
        .find(|(_, c)| *c == ' ')
        .expect("find whitespace")
        .0;

    let (first, second) = line.split_at(white_space_pos);
    let name = second[1..].to_string();

    if first == "dir" {
        Ok(LsLine::Directory(name))
    } else {
        let size = first.parse::<u64>().expect("parse file size");
        Ok(LsLine::File(name, size))
    }
}

fn build_filesystem(commands: Vec<CommandType>) -> Filesystem {
    commands
        .iter()
        .fold(
            (Vec::new(), HashMap::new()),
            |(mut path, mut fs), cmd| match cmd {
                CommandType::Cd(dir) => {
                    path.push(dir.clone());
                    (path, fs)
                }
                CommandType::CdParentDir => {
                    path.pop();
                    (path, fs)
                }
                CommandType::Ls(node_list) => {
                    let node_list: Vec<Node> = node_list
                        .iter()
                        .map(|node| match node {
                            LsLine::Directory(name) => Node(name.clone(), NodeType::Directory),
                            LsLine::File(name, size) => Node(name.clone(), NodeType::File(*size)),
                        })
                        .collect();

                    fs.insert(path.clone(), node_list);

                    (path, fs)
                }
            },
        )
        .1
}

#[cfg(test)]
mod tests {
    use crate::{size_of_smallest_directory_to_delete, sum_of_directories};

    const EXAMPLE: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"#;

    const PUZZLE_INPUT: &str = include_str!("../../puzzle_inputs/day7.txt");

    #[test]
    fn solves_p1_example() {
        assert_eq!(sum_of_directories(EXAMPLE), 95437);
    }

    #[test]
    fn solves_p2_example() {
        assert_eq!(size_of_smallest_directory_to_delete(EXAMPLE), 24933642);
    }

    #[test]
    fn solves_p1() {
        assert_eq!(sum_of_directories(PUZZLE_INPUT), 1989474);
    }

    #[test]
    fn solves_p2() {
        assert_eq!(size_of_smallest_directory_to_delete(PUZZLE_INPUT), 1111607);
    }
}
