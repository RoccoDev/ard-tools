use std::borrow::Cow;

use anyhow::{anyhow, Result};
use ardain::{path::ArhPath, DirEntry};
use clap::Args;

use crate::InputData;

#[derive(Args)]
pub struct ListArgs {
    working_directory: Option<ArhPath>,
    /// Only print file and directory names
    #[arg(short, long)]
    raw: bool,
}

#[derive(Default)]
struct Table<'a> {
    rows: Vec<Vec<Cow<'a, str>>>,
    lens: Vec<usize>,
}

pub fn run(input: &InputData, args: ListArgs) -> Result<()> {
    let fs = input.load_fs()?;
    let wd = args.working_directory.unwrap_or_default();

    let dir = fs
        .get_dir(&wd)
        .ok_or_else(|| anyhow!("directory not found"))?;
    let DirEntry::Directory { children } = &dir.entry else {
        unreachable!()
    };

    if !args.raw {
        println!("In {wd}:\n");
    }

    let mut dirs = 0;
    let mut files = 0;

    let mut table = Table::default();

    if !args.raw {
        table.push_row(vec!["Name", "Type", "Size"]);
        table.push_row(vec!["----", "----", "----"]);
    }

    for child in children {
        match child.entry {
            DirEntry::File => {
                let file_size = fs
                    .get_file_info(&format!("{wd}/{}", child.name))
                    .unwrap()
                    .actual_size();
                table.push_row(vec![
                    child.name.to_string(),
                    "File".to_string(),
                    format!("{file_size}"),
                ]);
                files += 1;
            }
            DirEntry::Directory { .. } => {
                table.push_row(vec![&child.name, "Directory", "--"]);
                dirs += 1;
            }
        }
    }

    table.print();

    if !args.raw {
        println!("\n{dirs} directories, {files} files");
    }

    Ok(())
}

impl<'a> Table<'a> {
    fn push_row<S: Into<Cow<'a, str>>>(&mut self, row: impl IntoIterator<Item = S>) {
        let row: Vec<_> = row.into_iter().map(Into::into).collect();
        for (i, cell) in row.iter().enumerate() {
            if i >= self.lens.len() {
                self.lens.push(cell.len());
            } else {
                self.lens[i] = cell.len().max(self.lens[i]);
            }
        }
        self.rows.push(row);
    }

    fn print(self) {
        for row in self.rows {
            for (i, cell) in row.into_iter().enumerate() {
                print!("{:<1$}  ", cell, self.lens[i]);
            }
            println!();
        }
    }
}
