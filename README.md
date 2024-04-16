# krc to lrc
酷狗音乐 krc 歌词转 lrc 歌词文件

#### 用法

```
C:/Users/Users> krc2lrc.exe -h
Copyright (c) 2024 houkunlin All rights reserved.
krc2lrc 0.1.0

USAGE:
    krc2lrc.exe [FLAGS] [OPTIONS] --input <input>

FLAGS:
    -h, --help       Prints help information
    -r, --raw-krc    是否保存解码后的原始KRC内容
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input>                    krc 文件路径或者文件目录（递归解析所有子目录）
    -t, --interval-time <interval-time>    如果前一行歌词结束后，与下一行歌词开始时间间隔超过[N]毫秒，就插入一行空白歌词
                                           [default: 500]
    -d, --max-dept <max-dept>              遍历目录路径的最大深度，默认当前目录 [default: 0]
```

```
# 转换单个文件
C:/Users/Users> krc2lrc.exe -i 1.krc
Copyright (c) 2024 houkunlin All rights reserved.
decode "1.krc" to lrc file: "1.lrc" is success!

# 转换单个文件，并保存解码后的 krc 内容
C:/Users/Users> krc2lrc.exe -i 1.krc -r
Copyright (c) 2024 houkunlin All rights reserved.
decode "1.krc" to lrc file: "1.lrc" is success!
decode "1.krc" to lrc file: "1.krc.lrc" is success!

# 转换当前目录下所有 krc 文件
C:/Users/Users> krc2lrc.exe -i . -d 0
Copyright (c) 2024 houkunlin All rights reserved.
decode "./1.krc" to lrc file: "./1.lrc" is success!
decode "./2.krc" to lrc file: "./2.lrc" is success!

# 转换当前目录下所有 krc 文件和最深3个层级子目录的歌词文件
C:/Users/Users> krc2lrc.exe -i . -d 3

# 如果前一行歌词结束后，与下一行歌词开始时间间隔超过200毫秒，就插入一行新的歌词
C:/Users/Users> krc2lrc.exe -i . -t 200
```

#### 打包发布

```shell
cargo build --release
```

#### 参考

- [krc-rs](https://github.com/CGQAQ/krc-rs)