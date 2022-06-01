#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::fs::File;
use reqwest::Client;
use futures_util::StreamExt;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![download_update])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn download_update(window: tauri::Window, dir: String, tag: String) -> Result<(), String> {
    println!("Downloading update {}", &tag);
    let url = format!("https://mikuni.me/download/update/{}.zip", &tag);
    let path = format!("{}/mikuni-update-{}.zip", &dir, &tag);
    let res = Client::new()
        .get(&url).send()
        .await.or(Err(format!("Failed to download update")))?;
    let total_size = res.content_length()
        .ok_or(format!("Failed to download update file info"))?;

    let mut file = File::create(&path).or(Err(format!("Failed to create file '{}'", &path)))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        file.write_all(&chunk).or(Err(format!("Error while writing to file")))?;
        downloaded += chunk.len() as u64;
        window.emit("update_progress", Some(downloaded * 100 / total_size));
    }

    let out_dir = format!("{}/Game", &dir);
    println!("Installing update {}...", &tag);
    extract_files(&window, &out_dir, &path)?;
    if let Err(e) = fs::remove_file(&path) {
        eprintln!("Warning: Could not delete update zip file");
    }
    Ok(())
}

fn extract_files(window: &tauri::Window, target_dir: &str, path: &str) -> Result<(), String> {
    let file = fs::File::open(&path)
        .or(Err(format!("Unable to open update archive at {}", &path)))?;
    let mut archive = zip::ZipArchive::new(file).or(Err("Unable to open update archive"))?;
    let dir = Path::new(&target_dir);

    let total_files = archive.len();
    for i in 0..total_files {
        let mut file = archive.by_index(i)
            .or(Err(format!("Could not extract file {} from archive", i)))?;
        let outpath = match file.enclosed_name() {
            Some(path) => dir.clone().join(path),
            None => continue,
        };

        let filename = outpath.display();
        if (*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath)
                .or(Err(format!("Could not extract file {} from archive", &filename)))?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)
                        .or(Err(format!("Could not extract file {} from archive", &filename)))?;
                }
            }
            let mut outfile = File::create(&outpath)
                .or(Err(format!("Could not extract file {} from archive", &filename)))?;
            io::copy(&mut file, &mut outfile)
                .or(Err(format!("Could not extract file {} from archive", &filename)))?;
        }
        println!("File {} extracted to \"{}\"", i, filename);
        window.emit("install_progress", Some(i * 100 / total_files));
    }
    Ok(())
}