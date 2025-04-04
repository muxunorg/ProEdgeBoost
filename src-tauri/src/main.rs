// src/main.rs
use tauri::{LogicalSize, Manager};
fn main() {
        tauri::Builder::default()
            .setup(|app| {
                    let main_window = app.get_webview_window("main").unwrap();

                    // 获取显示器尺寸（这里使用 LogicalSize 已足够）
                    let monitor = main_window.current_monitor().unwrap().unwrap();
                    let screen_size = monitor.size();
                    let screen_width = screen_size.width as f64;
                    let screen_height = screen_size.height as f64;

                    // 设置窗口尺寸（使用 LogicalSize）
                    let window_width = (screen_width * 0.6).round() as u32;
                    let window_height = (screen_height * 0.6).round() as u32;
                    main_window.set_size(LogicalSize::new(window_width, window_height)).unwrap();

                    // 居中窗口（无需手动计算坐标）
                    main_window.center().unwrap();

                    Ok(())
            })
            .run(tauri::generate_context!())
            .expect("Error while running Tauri application");
}