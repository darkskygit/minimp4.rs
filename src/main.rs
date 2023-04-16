use std::fs::File;
use std::io::{self, Cursor, Read, Seek, SeekFrom};
use std::path::Path;

use minimp4::Mp4Muxer;

fn read_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    let mut buf = vec![];
    File::open(path)?.read_to_end(&mut buf)?;
    Ok(buf)
}

fn main() {
    let mut buffer = Cursor::new(vec![]);
    let mut mp4muxer = Mp4Muxer::new(&mut buffer);
    if let Ok(h264) = read_file("input.264") {
        let pcm = read_file("input.pcm").unwrap();
        mp4muxer.init_video(1280, 720, false, "h264 stream");
        // mp4muxer.write_video_with_fps(&h264, 28);
        mp4muxer.init_audio(128000,8000, 1);
        mp4muxer.write_video_with_audio(&h264, 30, &pcm);
    }
    // if let Ok(h265) = read_file("1.265") {
    //     mp4muxer.init_video(1920, 1200, false, "h265 stream");
    //     mp4muxer.write_video(&h265);
    // }
    mp4muxer.write_comment("test comment");
    mp4muxer.close();
    buffer.seek(SeekFrom::Start(0)).unwrap();
    let mut video_bytes = Vec::new();
    buffer.read_to_end(&mut video_bytes).unwrap();
    std::fs::write( Path::new("output.mp4"), video_bytes).unwrap();
}
