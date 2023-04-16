use minimp4_sys::{
    mp4_h26x_write_nal, mp4_h26x_writer_t, track_media_kind_t_e_audio, MP4E_add_track,
    MP4E_put_sample, MP4E_set_dsi, MP4E_track_t, MP4E_track_t__bindgen_ty_1,
    MP4E_track_t__bindgen_ty_1__bindgen_ty_1, MP4E_SAMPLE_RANDOM_ACCESS,
    MP4_OBJECT_TYPE_AUDIO_ISO_IEC_14496_3,
};
use std::convert::TryInto;
use std::os::raw::c_void;

use super::enc::{Encoder, EncoderParams};

fn get_nal_size(buf: &mut [u8], size: usize) -> usize {
    let mut pos = 3;
    while size - pos > 3 {
        if buf[pos] == 0 && buf[pos + 1] == 0 && buf[pos + 2] == 1 {
            return pos;
        }
        if buf[pos] == 0 && buf[pos + 1] == 0 && buf[pos + 2] == 0 && buf[pos + 3] == 1 {
            return pos;
        }
        pos += 1;
    }
    size
}

pub fn write_mp4(mp4wr: &mut mp4_h26x_writer_t, fps: i32, data: &[u8]) {
    let mut data_size = data.len();
    let mut data_ptr = data.as_ptr();

    while data_size > 0 {
        let buf = unsafe { std::slice::from_raw_parts_mut(data_ptr as *mut u8, data_size) };
        let nal_size = get_nal_size(buf, data_size);
        if nal_size < 4 {
            data_ptr = unsafe { data_ptr.add(1) };
            data_size -= 1;
            continue;
        }
        unsafe { mp4_h26x_write_nal(mp4wr, data_ptr, nal_size as i32, (90000 / fps) as u32) };
        data_ptr = unsafe { data_ptr.add(nal_size) };
        data_size -= nal_size;
    }
}

pub fn write_mp4_with_audio(
    mp4wr: &mut mp4_h26x_writer_t,
    fps: i32,
    data: &[u8],
    pcm: &[u8],
    encoder_params: EncoderParams,
) {
    let mut data_size = data.len();
    let mut data_ptr = data.as_ptr();

    let sample_rate = encoder_params.sample_rate;
    let channel_count = encoder_params.channel_count;

    let encoder = Encoder::new(encoder_params).unwrap();
    let info = encoder.info().unwrap();

    let language: [u8; 4] = [0x75, 0x6e, 0x64, 0x00]; // und\0
    let tr: MP4E_track_t = MP4E_track_t {
        object_type_indication: MP4_OBJECT_TYPE_AUDIO_ISO_IEC_14496_3,
        language,
        track_media_kind: track_media_kind_t_e_audio,
        time_scale: 90000,
        default_duration: 0,
        u: MP4E_track_t__bindgen_ty_1 {
            a: MP4E_track_t__bindgen_ty_1__bindgen_ty_1 {
                channelcount: channel_count,
            },
        },
    };

    let mux = mp4wr.mux;
    let audio_track_id = unsafe { MP4E_add_track(mux, &tr) };

    unsafe {
        MP4E_set_dsi(
            mux,
            audio_track_id,
            info.confBuf.as_ptr() as *const c_void,
            info.confSize.try_into().unwrap(),
        )
    };

    let length: u64 = if channel_count == 1 { 1024 } else { 2048 };
    let mut input_buffer = vec![0i16; length as usize];
    let mut output_buffer = vec![0u8; length as usize];

    let pcm_size: u64 = pcm.len().try_into().unwrap();

    let mut sample: u64 = 0;
    let total_samples: u64 = pcm_size;
    let mut ts: u64 = 0;
    let mut ats: u64 = 0;

    let in_args_num_in_samples = length;
    let mut pcm_ptr = pcm.as_ptr();

    while data_size > 0 {
        let buf = unsafe { std::slice::from_raw_parts_mut(data_ptr as *mut u8, data_size) };
        let nal_size = get_nal_size(buf, data_size);
        if nal_size < 4 {
            data_ptr = unsafe { data_ptr.add(1) };
            data_size -= 1;
            continue;
        }
        unsafe { mp4_h26x_write_nal(mp4wr, data_ptr, nal_size as i32, (90000 / fps) as u32) };
        data_ptr = unsafe { data_ptr.add(nal_size) };
        data_size -= nal_size;

        ts += 90000 / fps as u64;
        while ats < ts {
            let bytes_to_read = std::cmp::min(total_samples, in_args_num_in_samples);
            let bytes_read = bytes_to_read * 2; // 2 bytes per i16
            let pcm_buf = unsafe {
                std::slice::from_raw_parts(pcm_ptr as *const i16, bytes_to_read.try_into().unwrap())
            };
            // Copy PCM data into input buffer
            input_buffer[..bytes_to_read as usize].copy_from_slice(pcm_buf);
            pcm_ptr = unsafe { pcm_ptr.add(bytes_read.try_into().unwrap()) };

            // if total_samples < in_args_num_in_samples as u64 {
            //     total_samples = pcm_size ;
            // }

            // Encode audio data using AAC encoder
            match encoder.encode(&input_buffer[..bytes_to_read as usize], &mut output_buffer) {
                Ok(encoding_info) => {
                    // Write encoded audio data to output buffer
                    let buf = &output_buffer[..encoding_info.output_size];
                    sample += 1024;
                    // total_samples -= bytes_to_read;
                    ats = sample * 90000 / sample_rate as u64;
                    unsafe {
                        MP4E_put_sample(
                            mux,
                            audio_track_id,
                            buf.as_ptr() as *mut c_void,
                            encoding_info.output_size.try_into().unwrap(),
                            (1024 * 90000 / sample_rate as usize).try_into().unwrap(),
                            MP4E_SAMPLE_RANDOM_ACCESS.try_into().unwrap(),
                        )
                    };
                }
                Err(e) => {
                    println!("encode error:{}", e);
                    break;
                }
            }
        }
    }
}
