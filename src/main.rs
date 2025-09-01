use clap::Parser;
use console::Term;
use std::env;

use crate::krc_to_lrc_cli::decode_krc_path;

mod krc_to_lrc;
mod krc_to_lrc_cli;
mod tests;

#[derive(Debug, Parser)]
#[command(version, about = "酷狗音乐 krc 歌词转 lrc 歌词文件", long_about = None, author = "侯坤林 <houkunlin@aliyun.com>",)]
struct Cli {
    /// krc 文件路径或者文件目录（递归解析所有子目录）
    #[arg(short = 'i', long)]
    input: Option<std::path::PathBuf>,
    /// 如果前一行歌词结束后，与下一行歌词开始时间间隔超过[N]毫秒，就插入一行空白歌词
    #[arg(short = 't', long, default_value = "500")]
    interval_time: i32,
    /// 遍历目录路径的最大深度，默认当前目录
    #[arg(short = 'd', long, default_value = "0")]
    max_dept: i32,
    /// 是否保存解码后的原始KRC内容
    #[arg(short = 'r', long, default_value = "false")]
    raw_krc: bool,
    /// krc 文件路径或者文件目录（递归解析所有子目录）
    #[arg()]
    inputs: Vec<std::path::PathBuf>,
}

fn wait_for_key_press() {
    let term = Term::stdout();
    println!("\n按任意键继续...");
    term.read_key().unwrap(); // 读取单个按键
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Copyright (c) 2025 houkunlin All rights reserved.");
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        println!("\n没有传入任何参数，请使用 -h 命令查看参数说明");
        wait_for_key_press();
        return Ok(());
    }
    // let path = env::current_dir().unwrap();
    let cli = Cli::parse();
    // println!("{:?} {:?}", cli, path);
    let mut num: [i32; 2] = [0, 0];
    if let Some(input) = cli.input {
        num = decode_krc_path(input, cli.interval_time, cli.raw_krc, cli.max_dept);
    }
    if !cli.inputs.is_empty() {
        for input in cli.inputs {
            let num1 = decode_krc_path(input, cli.interval_time, cli.raw_krc, cli.max_dept);
            num[0] += num1[0];
            num[1] += num1[1];
        }
    }

    println!(
        "共涉及 {} 个文件，已成功转换 {} 个文件，失败 {} 个文件",
        num[0] + num[1],
        num[0],
        num[1]
    );

    Ok(())
}
