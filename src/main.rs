mod lib;

use lib::Mp4Muxer;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut mp4muxer = Mp4Muxer::new(File::create("1.mp4").unwrap());
    let mut h264 = Vec::new();
    File::open("1.264").unwrap().read_to_end(&mut h264).unwrap();
    let mut h265 = Vec::new();
    File::open("1.265").unwrap().read_to_end(&mut h265).unwrap();
    mp4muxer.init_video(1920, 1200, false, "test1");
    mp4muxer.write_video(&h264);
    mp4muxer.init_video(1920, 1200, true, "test2");
    mp4muxer.write_video(&h265);
    mp4muxer.write_comment("test comment");
    mp4muxer.close();
}
