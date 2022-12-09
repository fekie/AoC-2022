// Thank you for https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/
// showing how to make tree structures in Rust look nice.

/*
A rant:
this is probably the messiest aoc ive done, but i stopped caring an hour ago
im a little over 3 hours deep now, the code lost all elegant about an hour in
lesson learned: tree structures suck in rust
*/

use std::{collections::HashMap, default::Default};

const INPUT: &str = include_str!("../input.txt");

type INodeId = usize;

#[derive(Debug)]
enum Command {
    Cd { argument: String },
    Ls { output: Vec<String> },
}

/// A memory arena so we don't have to mess with yucky lifetimes.
/// This works because everything in the memory arena has the same lifetime.
/// https://en.wikipedia.org/wiki/Region-based_memory_management
///
/// BUT, it's nice to just name it FileSystem here since that's
/// what we're using it as.
#[derive(Debug)]
struct FileSystem {
    inodes: Vec<INode>,
}

#[derive(Debug, Clone)]
enum INodeDetails {
    Directory {
        path: String,
    },
    /// Contains the file size.
    File {
        path: String,
        size: u64,
    },
}

#[derive(Debug, Clone)]
struct INode {
    parent: Option<INodeId>,
    children: Vec<INodeId>,

    pub data: INodeDetails,
}

fn main() {
    /* let mut memory_arena = Arena::new();
    let root = memory_arena.new_root_node(INodeDetails::Directory);
    let node = memory_arena.attach_new_node(root, INodeDetails::File(64)); */

    let commands = parse_commands();
    let file_system = FileSystem::from_commands(&commands);

    //dbg!(file_system.inode_from_id(file_system.ids_of_all_dirs(0)[1]));
    let map = file_system.dir_size_map();

    let mut sum_above_size_100000 = 0;
    for (_, size) in &map {
        if *size <= 100000 {
            sum_above_size_100000 += size;
        }
    }

    println!("Sum Above Size 100,000: {}", sum_above_size_100000);

    // Part 2
    let root_size = file_system.size_of_inode(0) as i64;
    let total_size = 70_000_000;
    let need_available = 30_000_000;
    let must_delete_file_of_at_least_this_size = (total_size - need_available - root_size).abs();

    let mut smallest_size_above = total_size;
    for (_, size) in map {
        let difference = size as i64 - must_delete_file_of_at_least_this_size;

        if difference <= 0 {
            continue;
        }

        let leading_difference = smallest_size_above - must_delete_file_of_at_least_this_size;

        if difference < leading_difference {
            smallest_size_above = size as i64;
        }
    }

    println!("Smallest Size Above: {}", smallest_size_above);
}

fn parse_commands() -> Vec<Command> {
    let mut commands = Vec::new();

    // For the ls command, we use this buffer to store the output
    let mut output_buffer: Vec<String> = Vec::new();

    // We go throught the commands backwards to make outputs easier.
    for line in INPUT.lines().rev() {
        // Command line follows structure: $ cm path
        // Output line follows structure: size filename
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let is_command_line = split[0] == "$";

        match is_command_line {
            true => {
                let command = match split[1] {
                    "cd" => Command::Cd {
                        argument: split[2].to_string(),
                    },
                    "ls" => {
                        let command = Command::Ls {
                            output: output_buffer.clone(),
                        };

                        output_buffer.clear();

                        command
                    }
                    _ => panic!("Invalid command."),
                };

                commands.push(command);
            }
            false => {
                let joined = split.join(" ");
                output_buffer.push(joined);
            }
        };
    }

    // We reverse the commands list since we parsed it backwards.
    commands.reverse();

    commands
}

impl FileSystem {
    fn new() -> Self {
        Self { inodes: Vec::new() }
    }

    fn from_commands(command: &[Command]) -> Self {
        let mut file_system = FileSystem::new();

        let mut current_path = String::new();
        let mut current_parent: Option<INodeId> = None;

        for command in command {
            match command {
                Command::Cd { argument } => match current_parent {
                    Some(_) => {
                        //dbg!(current_path.clone());
                        current_path = match argument.as_str() {
                            ".." => {
                                let mut split = current_path
                                    .split('/')
                                    .filter(|x| !x.is_empty())
                                    .collect::<Vec<&str>>();

                                split.pop();

                                let joined = split.join("/");

                                match joined.is_empty() {
                                    true => "/".to_owned(),
                                    false => {
                                        format!("/{}/", joined)
                                    }
                                }
                            }
                            _ => format!("{}{}/", current_path, argument),
                        };

                        //dbg!(&file_system);
                        //dbg!(current_path.clone());
                        current_parent = Some(file_system.id_from_path(&current_path));
                    }
                    None => {
                        let inode_id = file_system.new_root_node(INodeDetails::Directory {
                            path: argument.to_string(),
                        });

                        current_path = argument.to_string();
                        current_parent = Some(inode_id);
                    }
                },
                Command::Ls { output } => {
                    let mut all_inode_details = Vec::new();

                    for line in output {
                        let split = line.split_whitespace().collect::<Vec<&str>>();
                        let is_dir = split[0] == "dir";

                        let inode_details = match is_dir {
                            true => {
                                let name = split[1];
                                let path = format!("{}{}/", current_path, name);
                                INodeDetails::Directory { path }
                            }
                            false => {
                                let size = split[0].parse::<u64>().unwrap();
                                let name = split[1];
                                let path = format!("{}{}/", current_path, name);

                                INodeDetails::File { path, size }
                            }
                        };

                        all_inode_details.push(inode_details);
                    }

                    for inode_details in all_inode_details {
                        file_system.attach_new_node(current_parent.unwrap(), inode_details);
                    }
                }
            };
        }

        file_system
    }

    fn new_root_node(&mut self, data: INodeDetails) -> INodeId {
        assert_eq!(self.inodes.len(), 0);

        self.inodes.push(INode {
            parent: None,
            children: Vec::new(),
            data,
        });

        0
    }

    fn attach_new_node(&mut self, parent_id: INodeId, data: INodeDetails) -> INodeId {
        // Returns the next index we can use.
        let next_id = self.inodes.len();

        self.inodes.push(INode {
            data,
            parent: Some(parent_id),
            children: Vec::new(),
        });

        //dbg!(&self.inodes[parent_id]);

        self.inodes[parent_id].children.push(next_id);

        next_id
    }

    // Will panic if id is out of bounds.
    fn inode_from_id(&self, id: INodeId) -> &INode {
        &self.inodes[id]
    }

    fn id_from_path(&self, path_to_be_searched: &str) -> INodeId {
        for (i, inode) in self.inodes.iter().enumerate() {
            match &inode.data {
                INodeDetails::File { path, .. } => {
                    if path == path_to_be_searched {
                        return i;
                    }
                }
                INodeDetails::Directory { path } => {
                    if path == path_to_be_searched {
                        return i;
                    }
                }
            }
        }

        panic!("No id found.")
    }

    /// If the inode is a file, it returns the size.
    /// If the inode is a directory, it returns the sum of
    /// all files in the directory (recursively).
    fn size_of_inode(&self, id: INodeId) -> u64 {
        let mut total_size = 0;

        match self.inode_from_id(id).data {
            INodeDetails::File { size, .. } => {
                total_size += size;
            }
            INodeDetails::Directory { .. } => {
                //dbg!(&self.inode_from_id(id).children);

                for child in &self.inode_from_id(id).children {
                    //dbg!(&self.inodes[*child]);
                    total_size += self.size_of_inode(*child);
                }
            }
        }

        total_size
    }

    // (recursive)
    fn ids_of_all_file_descendants(&self, id: INodeId) -> Vec<INodeId> {
        let mut ids = Vec::new();

        let original = self.inode_from_id(id);
        for child in &original.children {
            let inode = self.inode_from_id(id);
            match &inode.data {
                INodeDetails::File { .. } => ids.push(*child),
                INodeDetails::Directory { .. } => {
                    let mut new_ids = self.ids_of_all_file_descendants(*child);
                    ids.append(&mut new_ids);
                }
            }
        }

        ids
    }

    //(recursive)
    fn ids_of_all_dirs(&self, parent_id: INodeId) -> Vec<INodeId> {
        let mut ids = Vec::new();

        for child in &self.inodes[parent_id].children {
            let inode = self.inode_from_id(*child);

            match &inode.data {
                INodeDetails::File { .. } => {}
                INodeDetails::Directory { .. } => {
                    let mut new_ids = self.ids_of_all_dirs(*child);

                    ids.append(&mut new_ids);
                    ids.push(*child);
                }
            }
        }

        ids
    }

    /// Does not include root directory.
    fn dir_size_map(&self) -> HashMap<INodeId, u64> {
        let mut map = HashMap::new();
        let mut ids_of_dirs = self.ids_of_all_dirs(0);

        // Add the root as well.
        ids_of_dirs.push(0);
        //dbg!(ids_of_dirs.clone());

        for id in ids_of_dirs {
            map.insert(id, self.size_of_inode(id));
        }

        map
    }
}
