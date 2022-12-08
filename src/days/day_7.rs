use std::collections::BTreeMap;
use crate::days::internal_common::*;

#[derive(Debug)]
enum FileNode {
    Dir(FileNodeDir),
    File(FileNodeFile)
}

impl FileNode {
    fn is_dir(&self) -> bool
    {
        match self {
            Self::Dir(_) => true,
            _ => false,
        }
    }
}

type INode = usize;

#[derive(Debug, Default)]
struct FileNodeDir {
    children: BTreeMap<String, INode>
}

#[derive(Debug, Default)]
struct FileNodeFile {
    size: usize
}

fn get_dir_from_inode(nodes: &mut Vec<FileNode>, inode: INode) -> &mut FileNodeDir
{
    match &mut nodes[inode] {
        FileNode::Dir(dir) => dir,
        FileNode::File(_) => panic!("Wanted to get a dir but it is a file")
    }
}

fn get_size_recur(nodes: &Vec<FileNode>, inode: INode) -> usize
{
    match &nodes[inode] {
        FileNode::Dir(dir) => dir.children.iter().map(|(_, &inode)| get_size_recur(nodes, inode)).sum(),
        FileNode::File(file) => file.size
    }
}

fn build_file_tree<Input>(input: &mut Input) -> Result<Vec<FileNode>>
where Input: Read
{
    let mut nodes: Vec<FileNode> = Vec::new();
    nodes.push(FileNode::Dir(FileNodeDir::default()));
    let mut current_dir: INode = 0;
    let mut outer_dir_stack: Vec<INode> = Vec::new();
    do_for_each_command(input, |command, ls_output| {
        match command {
            Command::Cd(cd_args) => {
                let dest = cd_args.dest;
                current_dir = match dest {
                    CdDest::Inner(dest_name) => {
                        outer_dir_stack.push(current_dir);
                        let dest_inode = *get_dir_from_inode(&mut nodes, current_dir).children.get(&dest_name).unwrap();
                        let dest_node = &nodes[dest_inode];
                        match dest_node {
                            FileNode::File(_) => panic!("Cant cd to file {:?}", dest_name),
                            FileNode::Dir(_) => dest_inode
                        }
                    }
                    CdDest::Outer => outer_dir_stack.pop().unwrap(),
                    CdDest::Root => 0
                };
            },
            Command::Ls => {
                let ls_output = ls_output.unwrap();
                for ls_entry in ls_output {
                    match ls_entry {
                        LsEntry::Dir(dir_name) => {
                            nodes.push(FileNode::Dir(FileNodeDir::default()));
                            let new_inode = nodes.len() - 1;
                            get_dir_from_inode(&mut nodes, current_dir).children.insert(dir_name, new_inode);
                        },
                        LsEntry::File(file_entry) => {
                            nodes.push(FileNode::File(FileNodeFile {size: file_entry.size}));
                            let new_inode = nodes.len() - 1;
                            get_dir_from_inode(&mut nodes, current_dir).children.insert(file_entry.name, new_inode);
                        },
                    };
                }
            }
        }
        Ok(())
    })?;
    Ok(nodes)
}

pub fn day_7_part_1<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let nodes = build_file_tree(input)?;

    let mut sum = 0;
    for inode in 0..nodes.len() {
        if !nodes[inode].is_dir() {
            continue;
        }
        let size = get_size_recur(&nodes, inode);
        if size <= 100000 {
            sum += size;
        }
    }
    println!("Sum is {}", sum);

    Ok(())
}

fn do_for_each_command<I, F>(input: &mut I, mut func: F) -> Result<()>
where I: Read,
F: FnMut(Command, Option<Vec<LsEntry>>) -> Result<()>
{
    let content = get_whole_input_as_string(input)?;
    let mut remaining = content.as_str();
    while remaining.len() != 0 {
        let i = remaining;
        let (i, command) = parse::parse_command(i).unwrap();
        let (i, _) = parse::get_to_next_line(i).unwrap();
        let (i, command_output) = match command {
            Command::Ls => parse::parse_ls_entry_list(i).map(|(i, ls_out)| (i, Some(ls_out))).unwrap(),
            _ => (i, None)
        };
        remaining = i;
        func(command, command_output)?;
    }
    Ok(())
}

pub fn day_7_part_2<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let nodes = build_file_tree(input)?;

    let mut dir_sizes: Vec<usize> = Vec::new(); // excludes root
    for inode in 0..nodes.len() {
        if !nodes[inode].is_dir() {
            continue;
        }
        dir_sizes.push(get_size_recur(&nodes, inode))
    }
    dir_sizes.sort();

    let min_size = 30000000 - (70000000 - get_size_recur(&nodes, 0));
    let answer = dir_sizes.iter().find(|&&size| size >= min_size).unwrap();

    println!("Answer is {}", answer);

    Ok(())
}

#[derive(Debug)]
enum Command {
    Cd(CdArgs),
    Ls
}

#[derive(Debug)]
struct CdArgs {
    dest: CdDest
}

#[derive(Debug)]
enum CdDest {
    Root,
    Inner(String),
    Outer
}

#[derive(Debug)]
enum LsEntry {
    File(LsEntryFile),
    Dir(String)
}

#[derive(Debug)]
struct LsEntryFile {
    size: usize,
    name: String
}

mod parse {
    use crate::days::parse::*;
    use super::{Command, CdArgs, CdDest, LsEntry, LsEntryFile};
    use nom::{
        IResult,
        bytes::complete::{tag, take_till},
        character::complete::{alpha1, newline, space1},
        sequence::{tuple, preceded, terminated},
        branch::alt,
        combinator::map_res,
        multi::many0
    };

    fn parse_cd_cmd(i: &str) -> IResult<&str, Command>
    {
        let (i, dest) = preceded(tag("cd "), alt((
            map_res(tag("/"), |_| Ok::<CdDest, ()>(CdDest::Root)),
            map_res(tag(".."), |_| Ok::<CdDest, ()>(CdDest::Outer)),
            map_res(alpha1, |s: &str| Ok::<CdDest, ()>(CdDest::Inner(s.to_string())))
        )))(i)?;

        Ok((i,
            Command::Cd(CdArgs {dest})
        ))
    }

    fn parse_ls_cmd(i: &str) -> IResult<&str, Command>
    {
        map_res(tag("ls"), |_| Ok::<Command, ()>(Command::Ls))(i)
    }

    pub(super) fn parse_command(i: &str) -> IResult<&str, Command>
    {
        preceded(tag("$ "), alt((parse_cd_cmd, parse_ls_cmd)))(i)
    }

    fn parse_name(i: &str) -> IResult<&str, String>
    {
        map_res(take_till(|c| c == '\n'), |s: &str| Ok::<String, ()>(s.to_string()))(i)
    }

    pub(super) fn parse_ls_entry(i: &str) -> IResult<&str, LsEntry>
    {
        let parse_ls_entry_file = 
            map_res(
                tuple((parse_int::<usize, _>, preceded(space1, parse_name))),
                |(size, name)| Ok::<LsEntry, ()>(LsEntry::File(LsEntryFile { size, name }))
                );
        alt((
            map_res(preceded(tag("dir "), parse_name), |name| Ok::<LsEntry, ()>(LsEntry::Dir(name))),
            parse_ls_entry_file
        ))(i)
    }

    pub(super) fn get_to_next_line(i: &str) -> IResult<&str, &str>
    {
        terminated(take_till(|c| c == '\n'), newline)(i)
    }

    pub(super) fn parse_ls_entry_list(i: &str) -> IResult<&str, Vec<LsEntry>>
    {
        many0(terminated(parse_ls_entry, get_to_next_line))(i)
    }
}