use std::io::Read;

use flate2::read::ZlibDecoder;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    // KRC解码后的歌词单字的时间内容
    static ref LINE_WORD_RE:Regex = Regex::new(r"<\d+,\d+,\d+>").unwrap();
    // KRC解码后的歌词行开始和结束时间
    static ref LINE_TIME_RE:Regex = Regex::new(r"\[(\d+),(\d+)]").unwrap();
}

/**
原始代码：
const KRC_ENCODE_KEY = new Buffer(new Uint8Array([64, 71, 97, 119, 94, 50, 116, 71, 81, 54, 49, 45, 206, 210, 110, 105]))
const content: Buffer = ...;
const buffer = new Buffer(content.length - 4)
//解码
for (let i = 4; i < content.length; i++) {
    buffer[i - 4] = content[i] ^ KRC_ENCODE_KEY[(i - 4) % 16]
}
//解压
return zlib.unzipSync(buffer)
 */

/// 把数值转换成时间格式字符串。
/// 把 KRC 歌词时间转换成 LRC 歌词时间字符串
///
/// # Arguments
///
/// * `time`: 整型时间，歌词出现位置的毫秒数
///
/// returns: String 时间字符串：mm:ss
///
/// # Examples
///
/// ```
/// str_to_time(61234) = "[01:01.23]"
/// ```
pub fn str_to_time(time: i32) -> String {
    let mut result = String::new();
    result.push('[');
    if time >= 60_000 {
        result.push_str(&format!("{:02}", time / 60_000));
        result.push(':');
        result.push_str(&format!("{:02}", time % 60_000 / 1000));
    } else {
        result.push_str(&format!("00:{:02}", time / 1000));
    }
    result.push('.');
    result.push_str(&format!("{:02}", (time % 1000) / 10));
    result.push(']');
    result
}

/// 替换掉 KRC 歌词的字与字之间的时间格式内容
///
/// # Arguments
///
/// * `krc_raw`: KRC解码后的明文KRC歌词内容
///
/// returns: String
///
/// # Examples
///
/// ```
///
/// ```
pub fn replace_krc_raw(krc_raw: &str) -> String {
    let cow = LINE_WORD_RE.replace_all(krc_raw, "");

    cow.to_string()
}

/// 替换 KRC 歌词的时间内容为 LRC 格式
///
/// # Arguments
///
/// * `lrc_content`: LRC内容（但时间还是KRC格式）
/// * `new_line_interval_time`: 如果两行歌词时间间隔超过这个时间就插入一个空白歌词行
///
/// returns: String LRC歌词内容
///
/// # Examples
///
/// ```
///
/// ```
pub fn replace_krc_time(lrc_content: &str, new_line_interval_time: i32) -> String {
    let mut result = String::new();

    // 计算当前歌词所在时间
    let mut latest_time = 0;

    for line in lrc_content.lines() {
        let captures_option = LINE_TIME_RE.captures(line);

        if captures_option.is_none() {
            result.push_str(line);
            result.push('\n');
            continue;
        }

        let captures = captures_option.unwrap();

        let time_str_len = captures.get(0).unwrap().len();

        let start_time = captures.get(1).unwrap().as_str().to_string().parse::<i32>().unwrap();
        let time_len = captures.get(2).unwrap().as_str().to_string().parse::<i32>().unwrap();

        if latest_time < start_time - new_line_interval_time {
            // 当前歌词时间与新行歌词时间超过 new_line_interval 毫秒，传入一个空白歌词行
            result.push_str(&str_to_time(latest_time));
            result.push('\n');
        }
        latest_time = start_time + time_len;

        result.push_str(&str_to_time(start_time));
        result.push_str(&line[time_str_len..]);
        result.push('\n');
    }

    result
}

/// 解码 KRC 文件密文内容为明文 KRC 明文歌词内容
///
/// # Arguments
///
/// * `krc_bytes`: KRC 原文件的内容
///
/// returns: String KRC 解码后的明文内容
///
/// # Examples
///
/// ```
///
/// ```
pub fn decode_krc(krc_bytes: Vec<u8>) -> String {
    let krc_encode_key = vec![64, 71, 97, 119, 94, 50, 116, 71, 81, 54, 49, 45, 206, 210, 110, 105];
    let mut buffer = Vec::with_capacity(krc_bytes.len() - 4);
    for i in 4..krc_bytes.len() {
        // let v1 = krc_bytes.get(i).unwrap();
        // let v2 = krc_encode_key.get((i - 4) % 16).unwrap();
        // let x = v1 ^ v2;
        // buffer.push(x);
        buffer.push(krc_bytes[i] ^ krc_encode_key[(i - 4) % 16]);
    }

    let mut krc_raw = String::new();

    // 解压数据，参考了 https://github.com/CGQAQ/krc-rs/blob/master/src/parser.rs#L24
    let mut d = ZlibDecoder::new(&buffer[..]);
    d.read_to_string(&mut krc_raw).expect("decode krc bytes failed");

    // let x = buffer.as_slice();
    //// libflate = "2.0.0"
    // let mut d1 = Decoder::new(&x[..]).unwrap();
    // d1.read_to_string(&mut krc_raw).expect("decode krc bytes failed");

    krc_raw
}

/// 把 KRC 原文件内容解码后并转换成 LRC 格式歌词内容
///
/// # Arguments
///
/// * `krc_bytes`: KRC 原文件密文内容
/// * `new_line_interval_time`: 如果两行歌词时间间隔超过这个时间就插入一个空白歌词行
///
/// returns: String LRC 格式歌词内容
///
/// # Examples
///
/// ```
///
/// ```
#[allow(dead_code)]
pub fn krc_to_lrc(krc_bytes: Vec<u8>, new_line_interval_time: i32) -> String {
    let krc_raw = decode_krc(krc_bytes);

    let lrc_not_time = replace_krc_raw(&krc_raw);

    replace_krc_time(&lrc_not_time, new_line_interval_time)
}