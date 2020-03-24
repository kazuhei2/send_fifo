extern crate nix;
extern crate tempfile;

use nix::sys::stat;
use nix::unistd;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::thread::sleep;
use std::time;

fn main() {
    let fifo_path = Path::new("/tmp/foo.pipe");

    match unistd::mkfifo(fifo_path, stat::Mode::S_IRWXU) {
        Ok(_) => println!("created {:?}", fifo_path),
        Err(err) => println!("Error creating fifo: {}", err),
    }

    let mut file = OpenOptions::new().write(true).open(&fifo_path).unwrap();
    file.write_all(b"Hello, world!").unwrap();
    println!("file.write_all() done");
    sleep(time::Duration::from_secs(3));
}
