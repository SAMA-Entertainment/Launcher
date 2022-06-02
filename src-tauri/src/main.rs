#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::fs;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
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
        .await.map_err(|_| "Failed to start download (Unreachable endpoint)".to_string())?;
    let total_size = res.content_length()
        .ok_or_else(|| "Failed to start download (No Content-Length header)".to_string())?;

    let mut file = File::create(&path)
        .map_err(|_| format!("Failed to create file '{}'", &path))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    let abort = Arc::new(AtomicBool::new(false));
    let abort_me = abort.clone();

    let unlisten = window.once("abort_update", move |_| {
        println!("Trying to abort update...");
        abort_me.store(true, Ordering::SeqCst);
    });

    while let Some(item) = stream.next().await {
        if abort.load(Ordering::Relaxed) {
            println!("Aborting update...");
            drop(stream); // Close stream
            drop(file); // Close file
            if let Err(e) = fs::remove_file(&path) {
                eprintln!("Warning: Could not delete update zip file: {}", e);
            }
            return Err("Download was aborted.".to_string());
        }
        let chunk = item.map_err(|_| "Error while downloading file".to_string())?;
        file.write_all(&chunk).map_err(|_| "Error while writing to file".to_string())?;
        downloaded += chunk.len() as u64;
        let _ = window.emit("update_progress", Some(downloaded * 100 / total_size));
    }

    window.unlisten(unlisten);

    let out_dir = format!("{}/Game", &dir);
    println!("Installing update {}...", &tag);
    extract_files(&window, &out_dir, &path)?;
    if let Err(e) = fs::remove_file(&path) {
        eprintln!("Warning: Could not delete update zip file: {}", e);
    }
    Ok(())
}

fn extract_files(window: &tauri::Window, target_dir: &str, path: &str) -> Result<(), String> {
    let file = fs::File::open(&path)
        .map_err(|_| format!("Unable to open update archive at {}", &path))?;
    let mut archive = zip::ZipArchive::new(file).or(Err("Unable to open update archive"))?;

    let total_files = archive.len();
    for i in 0..total_files {
        let mut file = archive.by_index(i)
            .map_err(|_| format!("Could not extract file {} from archive", i))?;
        let out_path = match file.enclosed_name() {
            Some(path) => Path::new(&target_dir).join(path),
            None => continue,
        };

        let filename = out_path.display();
        if (*file.name()).ends_with('/') {
            fs::create_dir_all(&out_path)
                .map_err(|_| format!("Could not extract file {} from archive", &filename))?;
        } else {
            if let Some(p) = out_path.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)
                        .map_err(|_| format!("Could not extract file {} from archive", &filename))?;
                }
            }
            let mut outfile = File::create(&out_path)
                .map_err(|_| format!("Could not extract file {} from archive", &filename))?;
            io::copy(&mut file, &mut outfile)
                .map_err(|_| format!("Could not extract file {} from archive", &filename))?;
        }
        println!("File {} extracted to \"{}\"", i, filename);
        let _ = window.emit("install_progress", Some(i * 100 / total_files));
    }
    Ok(())
}