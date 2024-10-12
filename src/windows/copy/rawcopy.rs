// Copyright 2021 Colin Finck <colin@reactos.org>
// SPDX-License-Identifier: MIT OR Apache-2.0
//
//! `RawCopy` crate provides the capability to use "Volume Shadow Copy technology" for file copying in Rust.  
//! Primarily aimed at replicating files that cannot be directly copied due to being in use.
//!
//! ```ignore
//!    rawcopy_rs::rawcopy(file_path, save_path)?;
//! ```
//!

use anyhow::{bail, Context, Result};


use ntfs::indexes::NtfsFileNameIndex;
use ntfs::structured_values::{
    NtfsFileName, NtfsFileNamespace,
};
use ntfs::{Ntfs, NtfsFile, NtfsReadSeek};

use std::collections::VecDeque;
use std::fs::{File, OpenOptions};

use std::io::{BufReader, Read, Seek, Write};
use std::path::{Component, PathBuf};

use std::{path};

use super::SectorReader;

struct CommandInfo<'n, T>
where
    T: Read + Seek,
{
    current_directory: Vec<NtfsFile<'n>>,
    current_directory_string: String,
    fs: T,
    ntfs: &'n Ntfs,
}
///
/// copy file from `file_path` to `save_path`
/// 
/// params: <p>`file_path` is the absolute path of the file must exist. </p>
/// <p><pre>`save_path` is the directory where the copied file will be saved.  
///             The directory must exist, and the file must not exist.  
///             The file name will be the same as the name of the file being copied.  
///             If it points to an NTFS filesystem image, then a suffix will be appended.</pre></p>
/// 
/// 
pub fn rawcopy(file_path: &str, save_path: &str) -> Result<()> {
    let path = path::PathBuf::from(file_path);

    // let path = path.canonicalize().unwrap();
    // path.components 左往右迭代
    let mut components: VecDeque<_> = path
        .components()
        .filter_map(|comp| {
            if Component::RootDir != comp {
                Some(comp.as_os_str().to_str().unwrap())
            } else {
                None
            }
        })
        .collect();

    let prefix = components.pop_front().unwrap();
    let file = components.pop_back().unwrap();

    let prefix = format!(r"\\.\{prefix}");// wait https://github.com/dylni/normpath update tobe remove this line.

    let f = File::open(prefix)?;
    let sr = SectorReader::new(f, 4096)?;
    let mut fs = BufReader::new(sr);
    let mut ntfs = Ntfs::new(&mut fs)?;
    ntfs.read_upcase_table(&mut fs)?;
    let current_directory = vec![ntfs.root_directory(&mut fs)?];

    let mut info = CommandInfo {
        current_directory,
        current_directory_string: String::new(),
        fs,
        ntfs: &ntfs,
    };

    for ele in components {
        // println!("{}", ele);
        cd(ele, &mut info)?;
    }

    get(file, save_path, &mut info)?;
    Ok(())
}
fn best_file_name<T>(
    info: &mut CommandInfo<T>,
    file: &NtfsFile,
    parent_record_number: u64,
) -> Result<NtfsFileName>
where
    T: Read + Seek,
{
    // Try to find a long filename (Win32) first.
    // If we don't find one, the file may only have a single short name (Win32AndDos).
    // If we don't find one either, go with any namespace. It may still be a Dos or Posix name then.
    let priority = [
        Some(NtfsFileNamespace::Win32),
        Some(NtfsFileNamespace::Win32AndDos),
        None,
    ];

    for match_namespace in priority {
        if let Some(file_name) =
            file.name(&mut info.fs, match_namespace, Some(parent_record_number))
        {
            let file_name = file_name?;
            return Ok(file_name);
        }
    }

    bail!(
        "Found no FileName attribute for File Record {:#x}",
        file.file_record_number()
    )
}
fn cd<T>(arg: &str, info: &mut CommandInfo<T>) -> Result<()>
where
    T: Read + Seek,
{
    if arg.is_empty() {
        return Ok(());
    }

    if arg == ".." {
        if info.current_directory_string.is_empty() {
            return Ok(());
        }

        info.current_directory.pop();

        let new_len = info.current_directory_string.rfind('\\').unwrap_or(0);
        info.current_directory_string.truncate(new_len);
    } else {
        let index = info
            .current_directory
            .last()
            .unwrap()
            .directory_index(&mut info.fs)?;
        let mut finder = index.finder();
        let maybe_entry = NtfsFileNameIndex::find(&mut finder, info.ntfs, &mut info.fs, arg);

        if maybe_entry.is_none() {
            println!("Cannot find subdirectory \"{arg}\".");
            return Ok(());
        }

        let entry = maybe_entry.unwrap()?;
        let file_name = entry
            .key()
            .expect("key must exist for a found Index Entry")?;

        if !file_name.is_directory() {
            println!("\"{arg}\" is not a directory.");
            return Ok(());
        }

        let file = entry.to_file(info.ntfs, &mut info.fs)?;
        let file_name = best_file_name(
            info,
            &file,
            info.current_directory.last().unwrap().file_record_number(),
        )?;
        if !info.current_directory_string.is_empty() {
            info.current_directory_string += "\\";
        }
        info.current_directory_string += &file_name.name().to_string_lossy();

        info.current_directory.push(file);
    }

    Ok(())
}
fn get<T>(file: &str, save_path: &str, info: &mut CommandInfo<T>) -> Result<()>
where
    T: Read + Seek,
{
    // Extract any specific $DATA stream name from the file.
    let (file_name, data_stream_name) = match file.find(':') {
        Some(mid) => (&file[..mid], &file[mid + 1..]),
        None => (file, ""),
    };

    // Compose the output file name and try to create it.
    // It must not yet exist, as we don't want to accidentally overwrite things.
    let output_file_name = if data_stream_name.is_empty() {
        file_name.to_string()
    } else {
        format!("{file_name}_{data_stream_name}")
    };
    let output_file_path = [save_path, &output_file_name].iter().collect::<PathBuf>();
    let mut output_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&output_file_path)
        .with_context(|| format!("Tried to open \"{output_file_name}\" for writing"))?;

    // Open the desired file and find the $DATA attribute we are looking for.
    let file = parse_file_arg(file_name, info)?;
    let data_item = match file.data(&mut info.fs, data_stream_name) {
        Some(data_item) => data_item,
        None => {
            println!("The file does not have a \"{data_stream_name}\" $DATA attribute.");
            return Ok(());
        }
    };
    let data_item = data_item?;
    let data_attribute = data_item.to_attribute()?;
    let mut data_value = data_attribute.value(&mut info.fs)?;

    // println!(
    //     "Saving {} bytes of data in \"{}\"...",
    //     data_value.len(),
    //     output_file_name
    // );
    let mut buf = [0u8; 4096];

    loop {
        let bytes_read = data_value.read(&mut info.fs, &mut buf)?;
        if bytes_read == 0 {
            break;
        }

        output_file.write_all(&buf[..bytes_read])?;
    }
    // println!("Done! save to {}", &output_file_path.to_str().unwrap());
    Ok(())
}
#[allow(clippy::from_str_radix_10)]
fn parse_file_arg<'n, T>(arg: &str, info: &mut CommandInfo<'n, T>) -> Result<NtfsFile<'n>>
where
    T: Read + Seek,
{
    if arg.is_empty() {
        bail!("Missing argument!");
    }

    if let Some(record_number_arg) = arg.strip_prefix('/') {
        let record_number = match record_number_arg.strip_prefix("0x") {
            Some(hex_record_number_arg) => u64::from_str_radix(hex_record_number_arg, 16),
            None => u64::from_str_radix(record_number_arg, 10),
        };

        if let Ok(record_number) = record_number {
            let file = info.ntfs.file(&mut info.fs, record_number)?;
            Ok(file)
        } else {
            bail!(
                "Cannot parse record number argument \"{}\"",
                record_number_arg
            )
        }
    } else {
        let index = info
            .current_directory
            .last()
            .unwrap()
            .directory_index(&mut info.fs)?;
        let mut finder = index.finder();

        if let Some(entry) = NtfsFileNameIndex::find(&mut finder, info.ntfs, &mut info.fs, arg) {
            let entry = entry?;
            let file = entry.to_file(info.ntfs, &mut info.fs)?;
            Ok(file)
        } else {
            bail!("No such file or directory \"{}\".", arg)
        }
    }
}