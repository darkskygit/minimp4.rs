use std::{
    ffi::OsStr,
    fs::{copy, create_dir_all, read_dir, read_to_string, remove_dir_all, write},
    io,
    path::Path,
    process::{Command, Output, Stdio},
};

pub fn cmd<I, S>(program: S, args: I) -> Result<Output, io::Error>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    Command::new(program)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
}

pub fn replace_identifier(
    src: String,
    identifier: &str,
    replacement: &str,
    offset: usize,
) -> String {
    let mut slice = &src[offset..];
    let mut slice_offset = offset;
    while let Some(index) = slice.find(identifier) {
        let src_index = slice_offset + index;
        let before_is_alphanumeric = src_index != 0
            && src[src_index - 1..]
                .chars()
                .next()
                .map_or(false, |c| c.is_alphanumeric() || c == '_');
        let after_is_alphanumeric = src_index + identifier.len() < src.len()
            && src[(src_index + identifier.len())..]
                .chars()
                .next()
                .map_or(false, |c| c.is_alphanumeric() || c == '_');
        if !before_is_alphanumeric && !after_is_alphanumeric {
            let new_src = String::from(&src[0..src_index])
                + replacement
                + &src[(src_index + identifier.len())..];
            return replace_identifier(new_src, identifier, replacement, src_index + 1);
        }
        slice = &slice[(index + 1)..];
        slice_offset = src_index + 1;
    }
    src
}

pub fn minimp4_c_dir() -> String {
    String::from("/minimp4")
}

pub fn run() {
    c_merge_files("/tmp/minimp4-merged.c");
    c_fixes_before_preprocessor(
        "/tmp/minimp4-merged.c",
        "/tmp/minimp4-fixes-before-preprocessor.c",
    );
    c_run_preprocessor(
        "/tmp/minimp4-fixes-before-preprocessor.c",
        "/tmp/minimp4-preprocessor.c",
    );
    c_fixes_after_preprocessor(
        "/tmp/minimp4-preprocessor.c",
        "/tmp/minimp4-fixes-after-preprocessor.c",
    );
    copy("/tmp/minimp4-fixes-after-preprocessor.c", "/out/minimp4.c").unwrap();

    c2rust(
        "/tmp/minimp4-fixes-after-preprocessor.c",
        "/tmp/minimp4-c2rust.rs",
    );

    rust_fixes("/tmp/minimp4-c2rust.rs", "/tmp/minimp4-fixes.rs");
    copy("/tmp/minimp4-fixes.rs", "/out/minimp4_c.rs").unwrap();
}

pub fn c_merge_files(output: &str) {
    let src_dir = minimp4_c_dir();
    let dir = read_dir(&src_dir).unwrap();
    let mut conglomerate = String::new();
    for file in dir {
        let name = String::from(file.unwrap().file_name().to_str().unwrap());
        if name.ends_with(".c") {
            let full_path = src_dir.clone() + "/" + name.as_str();
            conglomerate += &fix_source(&name, read_to_string(&full_path).unwrap());
        }
    }
    write(output, conglomerate).unwrap();
}

pub fn fix_source(name: &str, mut src: String) -> String {
    if name == "SkeletonJson.c" {
        src = replace_identifier(src, "_spLinkedMesh", "_spLinkedMeshJson", 0);
        src = replace_identifier(src, "setBezier", "setBezierJson", 0);
        src = replace_identifier(src, "readTimeline", "readTimelineJson", 0);
        src = replace_identifier(src, "readTimeline2", "readTimeline2Json", 0);
        src = replace_identifier(src, "readSequence", "readSequenceJson", 0);
        src = replace_identifier(src, "readCurve", "readCurveJson", 0);
        src = replace_identifier(src, "_readVertices", "_readVerticesJson", 0);
        src = replace_identifier(src, "string_starts_with", "string_starts_with_json", 0);
        src
    } else if name == "SkeletonBinary.c" {
        src = replace_identifier(src, "_spLinkedMesh", "_spLinkedMeshBinary", 0);
        src = replace_identifier(src, "setBezier", "setBezierBinary", 0);
        src = replace_identifier(src, "readTimeline", "readTimelineBinary", 0);
        src = replace_identifier(src, "readTimeline2", "readTimeline2Binary", 0);
        src = replace_identifier(src, "readSequence", "readSequenceBinary", 0);
        src = replace_identifier(src, "readCurve", "readCurveBinary", 0);
        src = replace_identifier(src, "_readVertices", "_readVerticesBinary", 0);
        src = replace_identifier(src, "string_starts_with", "string_starts_with_binary", 0);
        src
    } else if name == "AnimationState.c" {
        src = replace_identifier(src, "binarySearch1", "binarySearch1_state", 0);
        src
    } else {
        src
    }
}

pub fn c_fixes_before_preprocessor(input: &str, output: &str) {
    let mut src = read_to_string(input).unwrap();
    src = src.replace("isspace", "isspace_");
    write(
        output,
        String::from("typedef double _Float128;\nint isspace_(int x) { return x <= 32; }\n") + &src,
    )
    .unwrap();
}

pub fn c_run_preprocessor(input: &str, output: &str) {
    let src_dir = minimp4_c_dir();
    let include_dir = minimp4_c_dir();
    cmd(
        "gcc",
        [
            "-E",
            input,
            "-I",
            include_dir.as_str(),
            "-I",
            src_dir.as_str(),
            "-o",
            output,
        ],
    )
    .unwrap();
}

pub fn c_fixes_after_preprocessor(input: &str, output: &str) {
    let mut src = read_to_string(input).unwrap();
    src = replace_identifier(src, "memmove", "minimp4_memmove", 0);
    src = replace_identifier(src, "strlen", "minimp4_strlen", 0);
    src = replace_identifier(src, "memcpy", "minimp4_memcpy", 0);
    src = replace_identifier(src, "memset", "minimp4_memset", 0);
    src = replace_identifier(src, "strcpy", "minimp4_strcpy", 0);
    src = replace_identifier(src, "strcmp", "minimp4_strcmp", 0);
    src = replace_identifier(src, "strrchr", "minimp4_strrchr", 0);
    src = replace_identifier(src, "sqrtf", "minimp4_sqrtf", 0);
    src = replace_identifier(src, "strcasecmp", "minimp4_strcasecmp", 0);
    src = replace_identifier(src, "strncmp", "minimp4_strncmp", 0);
    src = replace_identifier(src, "strncat", "minimp4_strncat", 0);
    src = replace_identifier(src, "malloc", "minimp4_malloc", 0);
    src = replace_identifier(src, "realloc", "minimp4_realloc", 0);
    src = replace_identifier(src, "free", "minimp4_free", 0);
    src = replace_identifier(src, "strtol", "minimp4_strtol", 0);
    src = replace_identifier(src, "sprintf", "minimp4_sprintf", 0);
    src = replace_identifier(src, "printf", "minimp4_printf", 0);
    src = replace_identifier(src, "sscanf", "minimp4_sscanf", 0);
    src = replace_identifier(src, "strtoul", "minimp4_strtoul", 0);
    src = replace_identifier(src, "rand", "minimp4_rand", 0);
    src = replace_identifier(src, "fopen", "minimp4_fopen", 0);
    src = replace_identifier(src, "fseek", "minimp4_fseek", 0);
    src = replace_identifier(src, "ftell", "minimp4_ftell", 0);
    src = replace_identifier(src, "fread", "minimp4_fread", 0);
    src = replace_identifier(src, "fclose", "minimp4_fclose", 0);
    src = replace_identifier(src, "acosf", "minimp4_acosf", 0);
    src = replace_identifier(src, "atan2f", "minimp4_atan2f", 0);
    src = replace_identifier(src, "cosf", "minimp4_cosf", 0);
    src = replace_identifier(src, "pow", "minimp4_pow", 0);
    src = replace_identifier(src, "fmodf", "minimp4_fmodf", 0);
    write(output, src).unwrap();
}

pub fn c2rust(input: &str, output: &str) {
    let _ = remove_dir_all("/tmp/build");
    create_dir_all("/tmp/build").unwrap();
    let mut input_data = read_to_string(input).unwrap();
    input_data = input_data.replace("\"/tmp/minimp4-fixes-before-preprocessor.c\"", "\"minimp4.c\"");
    write("/tmp/build/minimp4.c", input_data).unwrap();
    write(
        "/tmp/build/compile_commands.json",
        r#"
        [
            {
                "directory": "/tmp/build",
                "command": "/usr/bin/cc -c /tmp/build/minimp4.c",
                "file": "/tmp/build/minimp4.c"
            }
        ]
    "#,
    )
    .unwrap();
    cmd("c2rust", ["transpile", "/tmp/build/compile_commands.json"]).unwrap();
    copy("/tmp/build/minimp4.rs", output).unwrap();
}

pub fn rust_fixes(input: &str, output: &str) {
    let mut src = read_to_string(input).unwrap();
    src = src.replace("libc::", "");
    src = src.replace("#![register_tool(c2rust)]\n", "");
    src = src.replace(
        "#![feature(extern_types, label_break_value, register_tool)]\n",
        "",
    );
    src = src.replace("pub type _IO_wide_data;", "");
    src = src.replace("pub type _IO_codecvt;", "");
    src = src.replace("pub type _IO_marker;", "");
    src += "\n
type _IO_wide_data = u8;
type _IO_codecvt = u8;
type _IO_marker = u8;
pub use crate::c::environment::types::*;
    ";
    src = src.replace(
        "fn minimp4_printf(__format: *const c_char, _: ...) -> c_int;",
        "",
    );
    src = src.replace(
        "fn minimp4_sprintf(\n        __s: *mut c_char,\n        __format: *const c_char,\n        _: ...\n    ) -> c_int;",
        "",
    );
    src = src.replace(
        "fn minimp4_sscanf(\n        __s: *const c_char,\n        __format: *const c_char,\n        _: ...\n    ) -> c_int;",
        "",
    );
    src = replace_identifier(src, "minimp4_printf", "minimp4_printf!", 0);
    src = replace_identifier(src, "minimp4_sprintf", "minimp4_sprintf!", 0);
    src = replace_identifier(src, "minimp4_sscanf", "minimp4_sscanf!", 0);
    write(output, src).unwrap();
}
