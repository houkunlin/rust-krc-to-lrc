use structopt::StructOpt;

use crate::krc_to_lrc_cli::decode_krc_path;

mod krc_to_lrc;
mod krc_to_lrc_cli;
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Copyright (c) 2024 houkunlin All rights reserved.");
    // let path = env::current_dir().unwrap();

    let args = Cli::from_args();
    // println!("{:?} {:?}", args, path);
    let num = decode_krc_path(args.input, args.interval_time, args.raw_krc, args.max_dept);

    println!(
        "共涉及 {} 个文件，已成功转换 {} 个文件，失败 {} 个文件",
        num[0] + num[1],
        num[0],
        num[1]
    );

    Ok(())
}
