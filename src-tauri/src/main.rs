use tauri::{LogicalSize, Manager};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = match app.get_webview_window("main") {
                Some(window) => window,
                None => {
                    eprintln!("无法找到主窗口");
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::NotFound, 
                        "主窗口不存在"
                    )));
                }
            };

            if let Some(monitor) = main_window.current_monitor()? {
                let scale_factor = main_window.scale_factor()?;

                let screen_size = monitor.size();

                let screen_width = ((screen_size.width as f64) / scale_factor).round() as u32;

                let _screen_height = ((screen_size.height as f64) / scale_factor).round() as u32;

                let desired_width = ((screen_width as f64) * 0.55).round() as u32;

                let desired_height = (((desired_width as f64) / (16.0 / 9.0))).round() as u32;

                main_window.set_size(LogicalSize::new(desired_width as f64, desired_height as f64)).unwrap();

                let min_width = ((1920.0_f64) * 0.55).round() as u32;

                let min_height = (((min_width as f64) / (16.0 / 9.0))).round() as u32;

                main_window.set_min_size(Some(LogicalSize::new(min_width as f64, min_height as f64))).unwrap();

                main_window.center()?;
            } else {
                eprintln!("No active monitor found.");
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}