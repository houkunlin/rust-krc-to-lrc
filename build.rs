fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();

        // 设置应用图标（如果存在）
        if std::path::Path::new("icon.ico").exists() {
            res.set_icon("icon.ico");
        }

        // 设置简体中文语言
        res.set_language(0x0804);

        // 编译资源
        if let Err(e) = res.compile() {
            eprintln!("资源编译失败: {}", e);
            // 继续编译，不因图标问题中断
        }
    }
}
