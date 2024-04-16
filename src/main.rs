use std::fmt::{Display, Formatter};
use std::fs;

use structopt::StructOpt;

use crate::krc_to_lrc::{decode_krc, replace_krc_raw, replace_krc_time};

mod krc_to_lrc;
mod tests;

#[derive(StructOpt, Debug)]
struct Cli {
    /// krc 文件路径或者文件目录（递归解析所有子目录）
    #[structopt(short = "i", long, parse(from_os_str))]
    input: std::path::PathBuf,
    /// 如果前一行歌词结束后，与下一行歌词开始时间间隔超过[N]毫秒，就插入一行空白歌词
    #[structopt(short = "t", long, default_value = "500")]
    interval_time: i32,
    /// 遍历目录路径的最大深度，默认当前目录
    #[structopt(short = "d", long, default_value = "0")]
    max_dept: i32,
    /// 是否保存解码后的原始KRC内容
    #[structopt(short = "r", long)]
    raw_krc: bool,
    // output krc file path
    // #[structopt(short = "o", long, parse(from_os_str))]
    // output: Option<std::path::PathBuf>,
}

#[derive(Debug)]
pub enum MyError {
    NotKrcFile(String),
}

impl std::error::Error for MyError {}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.to_string())
    }
}

/// 解码 KRC 文件为 LRC 内容
///
/// # Arguments
///
/// * `krc_file`: KRC 文件路径
/// * `new_line_interval_time`: 如果两行歌词时间间隔超过这个时间就插入一个空白歌词行
///
/// returns: Result<String, Box<dyn Error, Global>>
///
/// # Examples
///
/// ```
/// decode_krc_file_to_lrc("music.krc", 500)
/// ```
fn decode_krc_file(krc_file: &str) -> Result<String, Box<dyn std::error::Error>> {
    let result = fs::read(krc_file);

    let krc_bytes = match result {
        Ok(value) => { value }
        Err(error) => {
            println!("无法读取文件内容 {:?}", krc_file);
            return Err(error.into());
        }
    };

    // 前3个字节不是 krc 开头，则返回错误
    if !krc_bytes[0..3].eq(&vec![107, 114, 99]) {
        return Err(Box::new(MyError::NotKrcFile(format!("{:?} 错误的文件内容", krc_file))));
    }

    // let lrc_content = krc_to_lrc(krc_bytes, new_line_interval_time);

    Ok(decode_krc(krc_bytes))
}

/// 替换krc内容为lrc内容
///
/// # Arguments
///
/// * `krc_raw`: krc解码后的内容
/// * `new_line_interval_time`: 如果两行歌词时间间隔超过这个时间就插入一个空白歌词行
///
/// returns: String lrc歌词内容
///
/// # Examples
///
/// ```
///
/// ```
fn replace_krc_to_lrc(krc_raw: String, new_line_interval_time: i32) -> String {
    let lrc_not_time = replace_krc_raw(&krc_raw);

    replace_krc_time(&lrc_not_time, new_line_interval_time)
}

fn get_file_name(krc_file: &str) -> String {
    let path_len = krc_file.len();
    let new_file = if krc_file[path_len - 4..].to_lowercase().eq(".krc") {
        (&krc_file[0..path_len - 4]).to_owned()
    } else {
        krc_file.to_owned()
    };
    new_file
}

fn save_file(file_content: &str, krc_file: &str, lrc_file: String) {
    let result1 = fs::write(lrc_file.clone(), file_content);
    if result1.is_ok() {
        println!("decode {:?} to lrc file: {:?} is success!", krc_file.replace("\\", "/"), lrc_file.replace("\\", "/"));
    } else {
        println!("decode {:?} to lrc file: {:?} is error, can't write file", krc_file.replace("\\", "/"), lrc_file.replace("\\", "/"));
    }
}

/// 解码路径下文件
///
/// # Arguments
///
/// * `path`: 路径
/// * `new_line_interval_time`: 如果两行歌词时间间隔超过这个时间就插入一个空白歌词行
/// * `raw_krc`: 是否保存krc解码后文件
/// * `max_dept`: 最大路径深度
///
/// returns: [i32; 2] 下标 0 成功数量，下标 1 失败数量
///
/// # Examples
///
/// ```
///
/// ```
fn decode_path(path: std::path::PathBuf, new_line_interval_time: i32, raw_krc: bool, max_dept: i32) -> [i32; 2] {
    if max_dept < 0 {
        return [0, 0];
    }
    if path.is_file() {
        let krc_file = path.to_str().unwrap();

        // 解密 KRC 文件内容
        let result = decode_krc_file(krc_file);

        if result.is_err() {
            println!("Error: {:?}", result.err().unwrap());
            return [0, 1];
        }

        let file_name = get_file_name(krc_file);

        let krc_raw = result.unwrap();

        if raw_krc {
            save_file(&krc_raw, krc_file, file_name.clone() + ".krc.lrc");
        }

        // 替换 KRC 文件内容
        let lrc_content = replace_krc_to_lrc(krc_raw.clone(), new_line_interval_time);
        save_file(&lrc_content, krc_file, file_name.clone() + ".lrc");

        return [1, 0];
    } else if path.is_dir() {
        let mut num = [0, 0];
        let result = path.read_dir();
        if result.is_ok() {
            let dir = result.unwrap();
            for x in dir {
                if let Ok(dir_entry) = x {
                    let path_buf = dir_entry.path();
                    if path_buf.is_file() {
                        let pathname = path_buf.to_str().unwrap().to_lowercase();
                        if pathname.ends_with(".krc") {
                            let num_r = decode_path(path_buf, new_line_interval_time, raw_krc, max_dept);
                            num[0] += num_r[0];
                            num[1] += num_r[1];
                        }
                    } else if path_buf.is_dir() {
                        let num_r = decode_path(path_buf, new_line_interval_time, raw_krc, max_dept - 1);
                        num[0] += num_r[0];
                        num[1] += num_r[1];
                    }
                }
            }
        }
        return num;
    } else {
        println!("不支持的文件类型或者文件不存在");
    }
    return [0, 0];
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Copyright (c) 2024 houkunlin All rights reserved.");
    // let path = env::current_dir().unwrap();

    let args = Cli::from_args();
    // println!("{:?} {:?}", args, path);
    let num = decode_path(args.input, args.interval_time, args.raw_krc, args.max_dept);

    println!("共涉及 {} 个文件，已成功转换 {} 个文件，失败 {} 个文件", num[0] + num[1], num[0], num[1]);

    Ok(())
}
