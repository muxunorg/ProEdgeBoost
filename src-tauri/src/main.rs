// src/main.rs

use tauri::{LogicalSize, Manager, PhysicalPosition, PhysicalSize};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();

            // 获取当前显示器的尺寸
            let monitor = main_window.current_monitor().unwrap().unwrap();
            let screen_size = monitor.size();
            let screen_width = screen_size.width as f64;
            let screen_height = screen_size.height as f64;

            // 计算窗口大小（屏幕的 75%）
            let window_width = (screen_width * 0.55).round() as u32;
            let window_height = (screen_height * 0.55).round() as u32;

            // 设置窗口大小
            main_window
                .set_size(LogicalSize::new(window_width, window_height))
                .unwrap();

            // 居中窗口
            main_window.center().unwrap();

            // 显示窗口（如果配置文件里设置了初始不可见）
            main_window.show().unwrap();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}