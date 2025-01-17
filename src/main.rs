use std::io::{self};
use native_dialog::MessageDialog;

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


fn main() -> io::Result<()> {
    send_notification("Your backup is complete!")?;
    Ok(())
}
