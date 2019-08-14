mod lib;

use lib::Mp4Muxer;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut mp4muxer = Mp4Muxer::new(File::create("1.mp4").unwrap());
    let mut buf = Vec::new();
    File::open("1.264").unwrap().read_to_end(&mut buf).unwrap();
    mp4muxer.init_video(1920, 1200, false, "test1");
    mp4muxer.write_video(&buf);
    mp4muxer.init_video(1920, 1200, false, "test2");
    mp4muxer.write_video(&buf);
    mp4muxer.init_video(1920, 1200, false, "test3");
    mp4muxer.write_video(&buf);
    mp4muxer.init_video(1920, 1200, false, "test4");
    mp4muxer.write_video(&buf);
    mp4muxer.init_video(1920, 1200, false, "test5");
    mp4muxer.write_video(&buf);
    mp4muxer.close();
}
