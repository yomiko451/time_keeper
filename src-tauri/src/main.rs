// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![init, get_theme, show_main_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn show_main_window(window: tauri::Window) {
    window.get_window("main").unwrap().show().unwrap();
}

#[tauri::command]
fn init(app_handle: tauri::AppHandle) {
    let path = app_handle.path_resolver().app_local_data_dir().unwrap().join("theme.json");
    if !path.exists() {
        let file = std::fs::File::create(path).unwrap();
        serde_json::to_writer_pretty(file, &Theme::default()).unwrap();
    }
}

#[tauri::command]
fn get_theme(app_handle: tauri::AppHandle) -> Theme {
    let path = app_handle.path_resolver().app_local_data_dir().unwrap().join("theme.json");
    let file = std::fs::File::open(path).unwrap();
    let theme: Theme = serde_json::from_reader(file).unwrap();
    if theme.validate() {
        theme
    } else {
        Theme {
            background_color: "rgb(255, 255, 255)".to_string(),
            clock_board_color: "rgb(237, 237, 237)".to_string(),
            clock_graduation_color: "rgb(146, 147, 148)".to_string(),
            clock_hand_color: "rgb(52, 52, 53)".to_string(),
            clock_second_hand_color: "rgb(203, 1, 1)".to_string(),
            clock_progress_color: "rgb(209, 215, 220)".to_string(),
            button_background_color: "rgb(237, 237, 237)".to_string(),
            button_click_color: "rgb(200, 200, 200)".to_string(),
            menu_text_color: "rgb(0, 0, 0)".to_string(),
            menu_font_family: "宋体".to_string()
        }
    }
}


#[derive(serde::Serialize, serde::Deserialize, Default)]
struct Theme {
    background_color: String,
    clock_board_color: String,
    clock_graduation_color: String,
    clock_hand_color: String,
    clock_second_hand_color: String,
    clock_progress_color: String,
    button_background_color: String,
    button_click_color: String,
    menu_text_color: String,
    menu_font_family: String
}

fn is_valid_rgb(color: &str) -> bool {
    let re = regex::Regex::new(r"^rgb\((\d{1,3}),\s*(\d{1,3}),\s*(\d{1,3})\)$").unwrap();
    if let Some(caps) = re.captures(color) {
        caps.iter().skip(1).all(|cap| {
            cap.map_or(false, |m| {
                m.as_str().parse::<u8>().map_or(false, |n| (0..=255).contains(&n))
            })
        })
    } else {
        false
    }
}

impl Theme {
    fn validate(&self) -> bool {
        [
            &self.background_color,
            &self.clock_board_color,
            &self.clock_graduation_color,
            &self.clock_hand_color,
            &self.clock_second_hand_color,
            &self.clock_progress_color,
            &self.button_background_color,
            &self.button_click_color,
            &self.menu_text_color,
        ]
        .iter()
        .all(|color| is_valid_rgb(color))
    }
}

//单元测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_rgb() {
        assert!(is_valid_rgb("rgb(255, 255, 255)"));
        assert!(is_valid_rgb("rgb(0, 0, 0)"));
        assert!(!is_valid_rgb("rgb(128, 1288, 128)"));
        assert!(!is_valid_rgb("rbb(256, 256, 256)"))
   }
 }