#![feature(mem_copy_fn)]
mod disk;
use ext4::FileSystem;
use disk::DiskFile;
use clap::Parser;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// file of the disk path
    #[arg(short, long)]
    disk: String
}

fn main() {
    let args = Args::parse();
    let mut file = DiskFile::new(args.disk.as_str());
    let mut fs = FileSystem::new(&mut file).unwrap();
    println!("{}", fs.info());
    println!("/ {}", fs.root_inode().mode());
    println!("block count {}", fs.root_inode().blocks_count());


}
