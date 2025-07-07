use crate::krc_to_lrc::{decode_krc, replace_krc_raw, replace_krc_time};
use std::fmt::{Display, Formatter};
use std::fs;

#[derive(Debug)]
pub enum MyError {
    #[allow(dead_code)]
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
pub fn decode_krc_file(krc_file: &str) -> Result<String, Box<dyn std::error::Error>> {
    let result = fs::read(krc_file.replace("\\", "/"));

    let krc_bytes = match result {
        Ok(value) => value,
        Err(error) => {
            println!("无法读取文件内容 {:?}", krc_file);
            return Err(error.into());
        }
    };

    // 前3个字节不是 krc 开头，则返回错误
    if !krc_bytes[0..3].eq(&vec![107, 114, 99]) {
        return Err(Box::new(MyError::NotKrcFile(format!(
            "{:?} 错误的文件内容",
            krc_file
        ))));
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
pub fn replace_krc_to_lrc(krc_raw: String, new_line_interval_time: i32) -> String {
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
        println!(
            "decode {:?} to lrc file: {:?} is success!",
            krc_file.replace("\\", "/"),
            lrc_file.replace("\\", "/")
        );
    } else {
        println!(
            "decode {:?} to lrc file: {:?} is error, can't write file",
            krc_file.replace("\\", "/"),
            lrc_file.replace("\\", "/")
        );
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
pub fn decode_krc_path(
    path: std::path::PathBuf,
    new_line_interval_time: i32,
    raw_krc: bool,
    max_dept: i32,
) -> [i32; 2] {
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

        if krc_raw.len() == 0 {
            println!("decode {:?} krc file bytes failed", krc_file.replace("\\", "/"));
            return [0, 1];
        }

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
                            let num_r = decode_krc_path(
                                path_buf,
                                new_line_interval_time,
                                raw_krc,
                                max_dept,
                            );
                            num[0] += num_r[0];
                            num[1] += num_r[1];
                        }
                    } else if path_buf.is_dir() {
                        let num_r = decode_krc_path(
                            path_buf,
                            new_line_interval_time,
                            raw_krc,
                            max_dept - 1,
                        );
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
    [0, 0]
}
