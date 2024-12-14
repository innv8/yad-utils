use std::path::Path;

use chrono::Utc;

use crate::config;

#[derive(Debug)]
pub enum DownloadStatus {
    Pending,
    InProgress,
    Failed,
    Finished,
    Cancelled,
}

#[derive(Debug)]
pub enum FileType {
    Compressed,
    Videos,
    Audio,
    Documents,
    Programs,
    Images,
    Others,
}

#[derive(Debug)]
pub struct File {
    pub file_url: String,
    pub file_name: String,
    pub file_type: FileType,
    pub extension: String,
    pub destination_dir: String,
    pub destination_path: String,
    pub file_size: u64,
    pub download_start_time: i64,
    pub download_stop_time: i64,
    pub download_duration: i64,
    pub download_status: DownloadStatus
}

fn get_file_type(extension: &str) -> FileType {
    match extension {
         "mp4" | "mkv" | "avi" | "mov" | "flv" | "webm" | "wmv" | "mpeg" | "mpg" | "3gp" => FileType::Videos,
        "zip" | "rar" | "7z" | "tar" | "gz" | "targz" | "tarbz2" | "tarxz" | "iso" | "xz" => FileType::Compressed,
        "mp3" | "flac" | "wav" | "aac" | "ogg" | "m4a" | "wma" | "alac" | "opus" | "amr" => FileType::Audio,
        "pdf" | "docx" | "doc" | "txt" | "xlsx" | "pptx" | "ppt" | "odt" | "html" | "epub" | "csv" | "xml" => FileType::Documents,
        "exe" | "msi" | "bat" | "apk" | "dmg"  | "bin" | "deb" | "rpm"  => FileType::Programs,
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "webp" | "svg" | "ico" => FileType::Images,
        _ => FileType::Others,
    }
}

fn get_destination_path(file_name: &str,cfg: &config::Config, file_type: &FileType) -> (String, String) {
    let download_dir = Path::new(&cfg.download_dir);
    let dir = download_dir.join(format!("{:?}", file_type));
    let path = dir.join(file_name);

    let dir = dir.to_str().unwrap_or("_").to_string();
    let path = path.to_str().unwrap_or("_").to_string();


    (dir, path)
}

impl File {
    pub fn new(file_url: &str, cfg : &config::Config ) -> Self {
        let file_name = file_url.split('/')
            .last()
            .unwrap_or("");

        let extension =  file_name.split('.').last().unwrap_or("_").to_string();

        let file_type = get_file_type(&extension);
        let (destination_dir, destination_path) = get_destination_path(file_name, cfg, &file_type);

        File {
            file_url: file_url.to_string(),
            file_name: file_name.to_string(),
            file_type,
            extension,
            destination_dir,
            destination_path,
            file_size: 0,
            download_start_time:0,
            download_stop_time: 0,
            download_duration: 0,
            download_status: DownloadStatus::Pending,
        }
    }
    
}
