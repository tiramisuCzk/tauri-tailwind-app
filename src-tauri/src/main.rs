// 防止在 Windows 发布版本中出现额外的控制台窗口，不要删除此行!!
// 条件编译属性：如果不是调试构建，则设置 windows_subsystem 为 "windows"
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// 程序入口点函数
fn main() {
    // 调用 lib.rs 中定义的 run 函数启动 Tauri 应用
    tauri_app_lib::run()
}