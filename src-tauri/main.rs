// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![encrypt_files, decrypt_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use cipher::{decrypt_file, encrypt_file, get_now};
use std::path::Path;

#[tauri::command]
fn encrypt_files(files: Option<Vec<String>>, folder: Option<String>) -> String {
    if let Some(files_vec) = files {
        for file_path in &files_vec {
            let mut build_location = String::new();
            match folder {
                Some(ref folder_path) => {
                    build_location.clear();
                    build_location.push_str(&folder_path);
                }
                None => {}
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

            match parent {
                Some(ref parent_path) => {
                    build_location.clear();
                    build_location.push_str(&parent_path);
                }
                None => {}
            }

            let out_file_path = Path::new(&build_location);
            let now = get_now();
            let out_file_name = format!("{now}encryptout{file_stem}{extension}");
            let complete_path_option = out_file_path
                .join(out_file_name)
                .into_os_string()
                .into_string();

            let mut complete_path = String::new();
            if let Ok(complete_path_str) = complete_path_option {
                complete_path.clear();
                complete_path.push_str(&complete_path_str);
            };

            encrypt_file(file_path, "123", &complete_path);
        }
    }

    return String::from("null");
}

#[tauri::command]
fn decrypt_files(files: Option<Vec<String>>, folder: Option<String>) -> String {
    if let Some(files_vec) = files {
        for file_path in &files_vec {
            let mut build_location = String::new();
            match folder {
                Some(ref folder_path) => {
                    build_location.clear();
                    build_location.push_str(&folder_path);
                }
                None => {}
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

            match parent {
                Some(ref parent_path) => {
                    build_location.clear();
                    build_location.push_str(&parent_path);
                }
                None => {}
            }

            let out_file_path = Path::new(&build_location);
            let now = get_now();
            let out_file_name = format!("{now}decryptout{file_stem}{extension}");
            let complete_path_option = out_file_path
                .join(out_file_name)
                .into_os_string()
                .into_string();

            let mut complete_path = String::new();
            if let Ok(complete_path_str) = complete_path_option {
                complete_path.clear();
                complete_path.push_str(&complete_path_str);
            };

            decrypt_file(file_path, "123", &complete_path);
        }
    }

    return String::from("null");
}

// use same_file::Handle;
// use std::fs::{self, File};
// use std::io::Read;
// use std::io::{Error, ErrorKind};

// fn read_file(file_path: &String) -> Result<(), Error> {
//     let stdout_handle = Handle::stdout()?;
//     let handle = Handle::from_path(&file_path)?;

//     if stdout_handle == handle {
//         return Err(Error::new(
//             ErrorKind::Other,
//             "You are reading and writing to the same file",
//         ));
//     } else {
//         let a = get_file_as_byte_vec(file_path);
//         let e = encrypt_bytes(&a, "123");
//         println!("flie bytes: {:?}", a);
//         match e {
//             ResultBytesType::Success(sr) => {
//                 println!("e sr bytes: {:?}", sr);
//                 let d = decrypt_bytes(&sr, "123");

//                 if let ResultBytesType::Success(ref sr) = d {
//                     println!("d sr bytes: {:?}", sr)
//                 }
//                 match d {
//                     ResultBytesType::Success(sr) => {
//                         println!("d sr bytes: {:?}", sr)
//                     }
//                     ResultBytesType::Error(er) => println!("d er: {:?}", er),
//                 }
//             }
//             ResultBytesType::Error(er) => println!("e er: {:?}", er),
//         }
//         println!("end flie bytes: {:?}", a);
//     }

//     Ok(())
// }

// fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
//     let mut f = File::open(&filename).expect("no file found");
//     let metadata = fs::metadata(&filename).expect("unable to read metadata");
//     let mut buffer = vec![0; metadata.len() as usize];
//     f.read(&mut buffer).expect("buffer overflow");

//     buffer
// }
