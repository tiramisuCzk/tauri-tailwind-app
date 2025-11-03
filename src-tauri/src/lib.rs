// 了解更多关于 Tauri 命令的信息: https://tauri.app/develop/calling-rust/
// 使用 tauri::command 属性标记这是一个可以从前端调用的命令
#[tauri::command]
// 定义一个 greet 函数，接收一个字符串引用参数 name，返回格式化的问候语
fn greet(name: &str) -> String {
    // 使用 format! 宏格式化字符串并返回
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 使用条件编译属性，如果是移动端则设置为入口点
#[cfg_attr(mobile, tauri::mobile_entry_point)]
// 定义公共的 run 函数，作为 Tauri 应用的启动入口
pub fn run() {
    // 创建 Tauri 应用构建器
    tauri::Builder::default()
        // 初始化 opener 插件，用于打开文件或链接
        .plugin(tauri_plugin_opener::init())
        // 注册可以从前端调用的函数处理器
        .invoke_handler(tauri::generate_handler![greet])
        // 运行应用，加载上下文配置
        .run(tauri::generate_context!())
        // 如果运行过程中出现错误，则 panic 并显示错误信息
        .expect("error while running tauri application");
}