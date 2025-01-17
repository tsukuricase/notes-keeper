use chrono::Local;
use native_dialog::MessageDialog;
use std::fs::{self, File};
use std::io::{self};
use std::path::Path;
use zip::write::FileOptions;
use zip::ZipWriter;

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

fn add_folder_to_zip(
    zip: &mut ZipWriter<File>,
    root: &Path,
    path: &Path,
    options: &FileOptions,
) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        let relative_path = entry_path.strip_prefix(root).unwrap();

        if entry_path.is_dir() {
            zip.add_directory(relative_path.to_string_lossy(), *options)?;
            add_folder_to_zip(zip, root, &entry_path, options)?;
        } else {
            zip.start_file(relative_path.to_string_lossy(), *options)?;
            let mut file = File::open(&entry_path)?;
            io::copy(&mut file, zip)?;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let notes_folder = Path::new("xxxxx");
    let backup_folder = Path::new("xxxxx");

    if !backup_folder.exists() {
        fs::create_dir_all(&backup_folder)?;
    }

    let today = Local::now().format("%Y-%m-%d").to_string();
    let filename = format!("markdown-{}.zip", today);
    let zip_filename = backup_folder.join(&filename);

    let file = File::create(&zip_filename)?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    add_folder_to_zip(&mut zip, notes_folder, notes_folder, &options)?;

    zip.finish()?;

    let log_message = format!("Backup completed: {}", filename);
    send_notification(&log_message)?;

    Ok(())
}
