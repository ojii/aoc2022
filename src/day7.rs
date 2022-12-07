use itertools::Itertools;
use std::collections::HashMap;
use traversal::DftPre;

#[derive(Default, Debug)]
struct Directory {
    children: HashMap<String, Directory>,
    files: HashMap<String, usize>,
}

impl Directory {
    fn add_child(&mut self, name: String) {
        self.children.insert(name, Directory::default());
    }

    fn add_file(&mut self, name: String, size: usize) {
        self.files.insert(name, size);
    }

    fn size(&self) -> usize {
        self.files.values().sum::<usize>()
            + self.children.values().map(Directory::size).sum::<usize>()
    }

    fn get_child(&mut self, name: &String) -> Option<&mut Directory> {
        self.children.get_mut(name)
    }

    fn walk(&self) -> impl Iterator<Item = &Directory> {
        DftPre::new(self, |d| d.children.values()).map(|(_, d)| d)
    }
}

struct DirectoryWalker<'a, I: Iterator<Item = &'a Directory>> {
    dir: &'a Directory,
    children: I,
}

#[derive(Default, Debug)]
struct Shell {
    cwd: Vec<String>,
    fs: Directory,
}

impl Shell {
    fn new_with_script(script: &str) -> Self {
        let mut shell = Shell::default();
        shell.execute(script);
        shell
    }

    fn sum_directories<P: Fn(&Directory) -> bool>(&self, predicate: P) -> usize {
        self.fs
            .walk()
            .fold(0, |acc, d| if predicate(d) { d.size() + acc } else { acc })
    }

    fn execute(&mut self, script: &str) {
        for line in script.lines() {
            self.execute_line(line)
        }
    }

    fn execute_line(&mut self, line: &str) {
        match line.chars().next() {
            Some('$') => self.execute_command(&line[2..]),
            _ => self.handle_output(&line),
        }
    }

    fn execute_command(&mut self, command: &str) {
        let bits = command.split(' ').collect_vec();
        match bits[..] {
            ["cd", "/"] => {
                self.cwd.clear();
            }
            ["cd", ".."] => {
                self.cwd.pop();
            }
            ["cd", dir] => {
                self.cwd.push(dir.to_string());
            }
            ["ls"] => (),
            _ => panic!("unexpected command {}", command),
        };
    }

    fn handle_output(&mut self, line: &str) {
        let bits = line.split(' ').collect_vec();
        match bits[..] {
            ["dir", name] => {
                self.create_directory(name);
            }
            [num, name] if num.chars().all(|c| c.is_ascii_digit()) => {
                self.add_file(name, num.parse::<usize>().unwrap());
            }
            _ => panic!("Unexpected output {}", line),
        };
    }

    fn get_cwd(&mut self) -> &mut Directory {
        self.cwd
            .iter()
            .fold(&mut self.fs, |fs: &mut Directory, name| {
                fs.get_child(name).unwrap()
            })
    }

    fn create_directory(&mut self, name: &str) {
        self.get_cwd().add_child(name.to_string())
    }

    fn add_file(&mut self, name: &str, size: usize) {
        self.get_cwd().add_file(name.to_string(), size)
    }
}

const AVAILABLE_SPACE: usize = 70_000_000;
const NEEDED_SPACE: usize = 30_000_000;

fn find_smallest_directory_to_delete(fs: &Directory) -> usize {
    let used_space = fs.size();
    let target_space = AVAILABLE_SPACE - NEEDED_SPACE;
    let required_extra_space = used_space - target_space;
    fs.walk()
        .filter_map(|d| {
            let size = d.size();
            if size >= required_extra_space {
                Some(size)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

pub fn main() {
    let script = include_str!("data/day7");
    let shell = Shell::new_with_script(script);
    println!("{}", shell.sum_directories(|d| d.size() <= 100_000));
    println!("{}", find_smallest_directory_to_delete(&shell.fs));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "$ cd /
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
7214296 k";

    #[test]
    fn test_sum_small_dirs() {
        assert_eq!(
            95437,
            Shell::new_with_script(EXAMPLE).sum_directories(|d| d.size() <= 100_000)
        );
    }

    #[test]
    fn test_find_smallest_directory_to_delete() {
        assert_eq!(
            24933642,
            find_smallest_directory_to_delete(&Shell::new_with_script(EXAMPLE).fs)
        )
    }
}
