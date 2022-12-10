use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Directory {
    children: HashMap<String, Directory>,
    files: HashMap<String, u32>,
}

impl Directory {
    fn size(&self) -> u32 {
        let mut size = 0;
        for (_name, child) in &self.children {
            size += child.size();
        }
        for (_name, file_size) in &self.files {
            size += file_size;
        }
        size
    }

    fn directories_at_most_size(&self, size: u32) -> u32 {
        let mut count = if self.size() <= size { self.size() } else { 0 };
        for (_name, child) in &self.children {
            count += child.directories_at_most_size(size);
        }
        count
    }

    fn smallest_directory_at_least(&self, target_size: u32) -> u32 {
        let mut size = u32::MAX;
        if self.size() < size && self.size() >= target_size {
            size = self.size();
        }
        for (_name, child) in &self.children {
            let child_size = child.smallest_directory_at_least(target_size);
            if child_size < size && child_size >= target_size {
                size = child_size;
            }
        }
        size
    }
}

fn process(data: &str) -> u32 {
    let lines: Vec<&str> = data.split("\n").collect();
    let mut root = Directory {
        children: HashMap::new(),
        files: HashMap::new(),
    };
    let mut path = Vec::new();
    for line in lines {
        let line: Vec<&str> = line.split(" ").collect();
        if line[0] == "$" {
            match line[1] {
                "cd" => match line[2] {
                    "/" => path = Vec::new(),
                    ".." => path.truncate(path.len() - 1),
                    dir => path.push(dir),
                },
                "ls" => {}
                _ => panic!("Unknown command"),
            }
        } else {
            // Part of `ls` output
            let mut pwd = &mut root;
            for dir in path.iter() {
                pwd = pwd.children.get_mut(&dir.to_string()).unwrap();
            }
            if line[0] == "dir" {
                if pwd.children.contains_key(line[1]) {
                    println!("We've seen {} before", line[1]);
                } else {
                    pwd.children.insert(
                        line[1].to_string(),
                        Directory {
                            children: HashMap::new(),
                            files: HashMap::new(),
                        },
                    );
                }
            } else {
                pwd.files
                    .insert(line[1].to_string(), line[0].parse().unwrap());
            }
        }
    }
    let size_available = 70_000_000 - root.size();
    let size_needed = 30_000_000 - size_available;
    root.smallest_directory_at_least(size_needed) // directories_at_most_size() for part 1
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("{}", process(data));
}

#[cfg(test)]
mod test {
    use super::*;

    static DATA: &str = "$ cd /
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
    fn test() {
        assert!(process(DATA) == 24933642); // 95437 for part 1
    }
}
