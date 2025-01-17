# notes-keeper

A simple and efficient backup tool written in Rust to compress and backup your notes folder.

## Features
- 🗂️ Compress notes folder into a ZIP file.
- 📂 Save backups to a specified directory.
- 🔔 Desktop notifications on completion.
- 🚀 Support for incremental backups (future plan).

## Installation

1. Clone the repository:
```bash
git clone https://github.com/Jalever/notes-keeper.git
cd notes-keeper
```

2. Build the project:
```bash
cargo build --release
```

3. Run the tool:
 ```bash
./target/release/notes-keeper
```

## Configuration

1. Copy the example config file:
```bash
cp config.example.toml config.toml
```

2. Edit `config.toml` with your paths:
```bash
[paths]
notes_folder = "/path/to/your/notes"
backup_folder = "/path/to/your/backup"
```
