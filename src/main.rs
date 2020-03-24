extern crate nix;
extern crate tempfile;

use nix::fcntl;
use nix::sys::stat;
use nix::unistd;
use std::thread::sleep;
use std::time;
use tempfile::tempdir;

fn main() {
    let tmp_dir = tempdir().unwrap();
    let fifo_path = tmp_dir.path().join("foo.pipe");

    match unistd::mkfifo(&fifo_path, stat::Mode::S_IRWXU) {
        Ok(_) => println!("created {:?}", fifo_path),
        Err(err) => println!("Error creating fifo: {}", err),
    }

    let wait_ms = time::Duration::from_millis(1000);
    let fd = fcntl::open(&fifo_path, fcntl::OFlag::O_WRONLY, stat::Mode::S_IRWXU).unwrap();
    let data = "hello".as_bytes();
    loop {
        unistd::write(fd, &data).unwrap();
        sleep(wait_ms);
    }
}
