mod lib;

use lib::Mp4Muxer;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut mp4muxer = Mp4Muxer::new(File::create("1.mp4").unwrap());
    mp4muxer.init(316, 342, true);
    let mut buf = Vec::new();
    File::open("1.265").unwrap().read_to_end(&mut buf).unwrap();
    mp4muxer.write_mp4(&buf);
    mp4muxer.close();
}
