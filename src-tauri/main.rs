// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![encrypt_files, decrypt_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use cipher::{get_now, split_decrypt_file, split_encrypt_file};
use std::path::Path;

#[tauri::command]
fn encrypt_files(files: Option<Vec<String>>, folder: Option<String>) -> String {
    if let Some(files_vec) = files {
        for file_path in &files_vec {
            let mut build_location = String::new();

            let path = Path::new(file_path);
            let parent = path.parent().and_then(|value| value.to_str());

            // 不设置文件夹，使用文件父路径
            if let Some(parent_path) = parent {
                build_location.clear();
                build_location.push_str(parent_path);
            }

            if let Some(ref folder_path) = folder {
                build_location.clear();
                build_location.push_str(folder_path);
            }

            let file_stem_option = path.file_stem().and_then(|value| value.to_str());
            let mut file_stem = String::new();
            if let Some(file_stem_str) = file_stem_option {
                file_stem.clear();
                file_stem.push_str(file_stem_str);
            };

            let extension_option = path.extension().and_then(|value| value.to_str());
            let mut extension = String::new();
            if let Some(extension_str) = extension_option {
                extension.clear();
                extension.push_str(extension_str);
            };

            let out_file_path = Path::new(&build_location);
            let now = get_now();
            let out_file_name = format!("{now}encryptout{file_stem}{extension}");
            let complete_path_option = out_file_path.join(out_file_name).into_os_string().into_string();

            let mut complete_path = String::new();
            if let Ok(complete_path_str) = complete_path_option {
                complete_path.clear();
                complete_path.push_str(&complete_path_str);
            };

            split_encrypt_file(file_path, "123", &complete_path);
        }
    }

    String::from("null")
}

#[tauri::command]
fn decrypt_files(files: Option<Vec<String>>, folder: Option<String>) -> String {
    if let Some(files_vec) = files {
        for file_path in &files_vec {
            let mut build_location = String::new();
            if let Some(ref folder_path) = folder {
                build_location.clear();
                build_location.push_str(folder_path);
            }

            let path = Path::new(file_path);
            let parent = path.parent().and_then(|value| value.to_str());

            let file_stem_option = path.file_stem().and_then(|value| value.to_str());
            let mut file_stem = String::new();
            if let Some(file_stem_str) = file_stem_option {
                file_stem.clear();
                file_stem.push_str(file_stem_str);
            };

            let extension_option = path.extension().and_then(|value| value.to_str());
            let mut extension = String::new();
            if let Some(extension_str) = extension_option {
                extension.clear();
                extension.push_str(extension_str);
            };

            if let Some(parent_path) = parent {
                build_location.clear();
                build_location.push_str(parent_path);
            }

            let out_file_path = Path::new(&build_location);
            let now = get_now();
            let out_file_name = format!("{now}decryptout{file_stem}{extension}");
            let complete_path_option = out_file_path.join(out_file_name).into_os_string().into_string();

            let mut complete_path = String::new();
            if let Ok(complete_path_str) = complete_path_option {
                complete_path.clear();
                complete_path.push_str(&complete_path_str);
            };

            split_decrypt_file(file_path, "123", &complete_path);
        }
    }

    String::from("null")
}
