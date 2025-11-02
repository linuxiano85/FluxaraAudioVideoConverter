//! Modulo per l'elenco dei formati audio/video supportati.



/// Restituisce un vettore di stringhe contenente i formati audio/video supportati.
pub fn list_formats() -> Vec<String> {
    let mut formats = Vec::new();
    formats.push("mp3".to_string());
    formats.push("aac".to_string());
    formats.push("flac".to_string());
    formats.push("wav".to_string());
    formats.push("ogg".to_string());
    formats.push("m4a".to_string());
    formats.push("wma".to_string());
    formats.push("mp4".to_string());
    formats.push("avi".to_string());
    formats.push("mkv".to_string());
    formats.push("webm".to_string());
    formats.push("mov".to_string());
    formats.push("flv".to_string());
    formats.push("wmv".to_string());
    formats
}
