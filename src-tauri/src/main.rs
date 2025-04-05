use tauri::{LogicalSize, Manager};

fn main() {
    // 使用 Tauri 的默认 Builder 来创建应用实例
    tauri::Builder::default()
        .setup(|app| {
            // 获取主窗口实例，如果获取失败则输出错误信息并退出
            let main_window = app.get_webview_window("main").expect("Failed to get main window");

            // 尝试获取当前活动的显示器，如果失败则输出错误信息
            if let Some(monitor) = main_window.current_monitor().expect("Failed to get current monitor") {
                // 获取主窗口的缩放因子，用于处理高分辨率显示器（如Retina屏）
                let scale_factor = main_window.scale_factor().expect("Failed to get scale factor");

                // 获取当前显示器的尺寸
                let screen_size = monitor.size();

                // 根据缩放因子调整屏幕宽度，并四舍五入到最近的整数
                let screen_width = ((screen_size.width as f64) / scale_factor).round() as u32;

                // 计算屏幕高度，虽然这里没有直接使用它，但为了完整性保留计算
                let _screen_height = ((screen_size.height as f64) / scale_factor).round() as u32;

                // 计算窗口的期望宽度，初始大小为屏幕宽度的0.55倍，并保持16:9比例
                let desired_width = ((screen_width as f64) * 0.55).round() as u32;

                // 根据期望宽度计算对应的期望高度，以保持16:9的比例
                let desired_height = (((desired_width as f64) / (16.0 / 9.0))).round() as u32;

                // 设置窗口尺寸，使用 LogicalSize 结构体来指定宽度和高度
                main_window.set_size(LogicalSize::new(desired_width as f64, desired_height as f64)).unwrap();

                // 计算最小窗口宽度，设定为1080p分辨率宽度的0.55倍，并显式指定类型为f64以避免编译器警告
                let min_width = ((1920.0_f64) * 0.55).round() as u32;

                // 根据最小宽度计算对应的最小高度，以保持16:9的比例
                let min_height = (((min_width as f64) / (16.0 / 9.0))).round() as u32;

                // 设置窗口的最小尺寸，防止用户将窗口缩小到不可用的尺寸
                main_window.set_min_size(Some(LogicalSize::new(min_width as f64, min_height as f64))).unwrap();

                // 居中窗口，使窗口在启动时位于屏幕中央
                main_window.center().unwrap();
            } else {
                // 如果没有找到活动的显示器，则输出错误信息
                eprintln!("No active monitor found.");
            }

            // 返回 Ok 表示 setup 函数成功执行
            Ok(())
        })
        // 运行 Tauri 应用程序，并生成上下文。如果应用程序运行失败，则输出错误信息
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}