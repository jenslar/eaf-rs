use std::{path::{Path, PathBuf}, process::Command};

use crate::errors::EafError;

pub struct FFmpeg {}

impl FFmpeg {
    /// Extract section of a media file with `start_ms` and `end_ms` timespan
    /// in milliseconds (ELAN's default time value).
    /// If succesful the path to the extracted media file is returned.
    /// 
    /// TODO verify that media_path exists, return err if not
    pub fn extract_timespan(
        media_path: &Path,
        start_ms: u64,
        end_ms: u64,
        custom_outpath: Option<&Path>,
        ffmpeg_path: Option<&Path>
    ) -> Result<PathBuf, EafError> {
        let ffmpeg = if let Some(p) = ffmpeg_path {
            p.to_owned()
        } else if cfg!(windows) {
            PathBuf::from("ffmpeg.exe")
        } else {
            PathBuf::from("ffmpeg")
        };
    
        let outpath = match custom_outpath {
            Some(p) => p.to_owned(),
            None => {// e.g. path/to/infile.mp4 -> path/to/infile_1000-14000.mp4
                media_path.with_file_name(format!("{}_{}-{}.{}",
                    media_path.file_stem()
                        .ok_or(EafError::FileNameMissing(media_path.display().to_string()))?
                        .to_string_lossy()
                        .to_string(),
                    start_ms,
                    end_ms,
                    media_path.extension()
                        .ok_or(EafError::FileExtensionMissing(media_path.display().to_string()))?
                        .to_string_lossy()
                        .to_string()
                ))
            }
        };
    
        Command::new(&ffmpeg)
            .args(&[
                "-loglevel", "fatal",
                "-guess_layout_max", "0", // ffmpeg will not guess channel layout
                "-bitexact", // ffmpeg will not include LIST metadata
                "-i", &media_path.display().to_string(),
                "-ss", &format!("{}", start_ms as f64/1000.0), // start point in ms
                "-t", &format!("{}", (end_ms - start_ms) as f64/1000.0), // duration from start point in ms
                "-c", "copy", // copy streams, no recoding, faster and preserves input quality
                &outpath.display().to_string()
            ])
            .output()?;
        
        Ok(outpath)
    }
    
    /// Extract a WAV-file from specified video file to the same dir as the video.
    /// Returns the path to extracted WAV-file.
    pub fn extract_wav(video_path: &Path, ffmpeg_path: Option<&Path>) -> Result<PathBuf, EafError> {
        let ffmpeg = if let Some(p) = ffmpeg_path {
            p.to_owned()
        } else if cfg!(windows) {
            PathBuf::from("ffmpeg.exe")
        } else {
            PathBuf::from("ffmpeg")
        };
        
        let wav_path = video_path.with_extension("wav");
    
        Command::new(&ffmpeg)
            .args(&[
                "-i", &video_path.display().to_string(),
                "-vn",
                &wav_path.display().to_string()
            ])
            .output()?;
    
        Ok(wav_path)
    }

    // // TODO find a way to remove ffprobe depenency. MP4 ok via mp4iter, but not other media formats
    // /// Returns media duration in milliseconds.
    // pub fn get_duration(media_file: &Path, ffprobe_path: Option<&Path>) -> Result<u64, EafError> {
    //     let ffprobe = if let Some(path) = ffprobe_path {
    //         path
    //     } else if cfg!(windows) {
    //         Path::new("ffprobe.exe")
    //     } else {
    //         Path::new("ffprobe")
    //     };

    //     // ffprobe -v error -show_entries format=duration -of default=noprint_wrappers=1:nokey=1 FILE
    //     let args = [
    //         "-v",
    //         "error",
    //         "-show_entries",
    //         "format=duration",
    //         "-of",
    //         "default=noprint_wrappers=1:nokey=1",
    //         &media_file.display().to_string(),
    //     ];

    //     let output = Command::new(ffprobe)
    //         .args(&args)
    //         .output()?.stdout; // or ::new().spawn() ?
    //     let duration: f64 = std::str::from_utf8(&output)?
    //         .trim()
    //         .parse()?;

    //     Ok((duration * 1000.0) as u64)
    // }
}
