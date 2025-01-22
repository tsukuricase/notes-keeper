use chrono::Local;
use native_dialog::MessageDialog;
use std::fs::{self, File};
use std::io::{self};
use std::path::Path;
use zip::write::FileOptions;
use zip::ZipWriter;
use serde::Deserialize;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Deserialize)]
struct Config {
    paths: Paths,
}

#[derive(Deserialize)]
struct Paths {
    notes_folder: String,
    backup_folder: String,
}

fn load_config() -> Config {
    let config_str = fs::read_to_string("config.toml").expect("Failed to read config.toml");
    toml::from_str(&config_str).expect("Failed to parse config.toml")
}

fn send_notification(message: &str) -> io::Result<()> {
    let result = MessageDialog::new()
        .set_title("Backup")
        .set_text(message)
        .show_confirm()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    if result {
        println!("User clicked OK");
    } else {
        println!("User clicked Cancel");
    }
    Ok(())
}

fn count_files(path: &Path) -> io::Result<u64> {
    let mut count = 0;
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_dir() {
            count += count_files(&entry_path)?;
        } else {
            count += 1;
        }
    }
    Ok(count)
}

fn add_folder_to_zip(
    zip: &mut ZipWriter<File>,
    root: &Path,
    path: &Path,
    options: &FileOptions,
    progress_bar: &ProgressBar,
) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        let relative_path = entry_path.strip_prefix(root).unwrap();

        if entry_path.is_dir() {
            zip.add_directory(relative_path.to_string_lossy(), *options)?;
            add_folder_to_zip(zip, root, &entry_path, options, progress_bar)?;
        } else {
            zip.start_file(relative_path.to_string_lossy(), *options)?;
            let mut file = File::open(&entry_path)?;
            io::copy(&mut file, zip)?;
            progress_bar.inc(1);
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let config = load_config();
    let notes_folder = Path::new(&config.paths.notes_folder);
    let backup_folder = Path::new(&config.paths.backup_folder);

    if !backup_folder.exists() {
        fs::create_dir_all(&backup_folder)?;
    }

    let total_files = count_files(notes_folder)?; // 计算总文件数
    let today = Local::now().format("%Y-%m-%d").to_string();
    let filename = format!("markdown-{}.zip", today);
    let zip_filename = backup_folder.join(&filename);

    let file = File::create(&zip_filename)?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    // 创建进度条
    let progress_bar = ProgressBar::new(total_files);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .progress_chars("#>-"));

    add_folder_to_zip(&mut zip, notes_folder, notes_folder, &options, &progress_bar)?;

    zip.finish()?;

    progress_bar.finish_with_message("压缩完成");

    let log_message = format!("Backup completed: {}", filename);
    send_notification(&log_message)?;

    Ok(())
}
