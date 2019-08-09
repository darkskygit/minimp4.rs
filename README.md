# minimp4.rs

A [minimp4](https://github.com/lieff/minimp4) Rust binding.

# Usage

``` rust
    let mut mp4muxer = Mp4Muxer::new(File::create("1.mp4").unwrap());
    mp4muxer.init(316, 342, false);
    let mut buf = Vec::new();
    File::open("1.264").unwrap().read_to_end(&mut buf).unwrap();
    mp4muxer.write_mp4(&buf);
    mp4muxer.close();
```

# TODO

- [ ] Support hevc mux (Now can mux but cannot play)
- [ ] Support multiple track
- [ ] Support audio track
- [ ] Support set metadata

# Contributing

Pull request :)
