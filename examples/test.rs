#![feature(mem_copy_fn)]
#![feature(slice_pattern)]

extern crate core;
mod disk;

use ext4::{FileSystem};
use disk::DiskFile;
use clap::Parser;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// file of the disk path
    #[arg(short, long, default_value = "hd.img")]
    disk: String
}

fn main() {
    let args = Args::parse();
    let disk = DiskFile::new(args.disk.as_str());
    let mut fs = FileSystem::new(Box::new(disk)).unwrap();
    let file_name = "/test/test.bin";
    let f = fs.open(file_name);
    match f {
        None => println!("{} not found", file_name),
        Some(mut f) => {
            println!("{} {} {}", f.mode(), f.name(), f.size());
            let mut buffer = vec![0u8; 4096];
            f.read(&mut buffer).unwrap();
            println!("{:?}", buffer);
        }
    }

}
