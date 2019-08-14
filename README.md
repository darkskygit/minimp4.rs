# minimp4.rs

A [minimp4](https://github.com/lieff/minimp4) Rust binding.

# Usage

``` rust
    let mut mp4muxer = Mp4Muxer::new(File::create("1.mp4").unwrap());
    let mut buf = Vec::new();
    File::open("1.264").unwrap().read_to_end(&mut buf).unwrap();
    mp4muxer.init_video(316, 342, false, "title");
    mp4muxer.write_video(&buf);
    mp4muxer.close();
```

# TODO

- [ ] Support hevc mux (Now can mux but cannot play)
- [x] Support multiple track
- [ ] Support audio track
- [x] Support set track title
- [ ] Support set metadata

# Contributing

Pull request :)
