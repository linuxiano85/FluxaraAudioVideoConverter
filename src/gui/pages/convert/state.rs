//! Modulo per la gestione dello stato della pagina di conversione.


/// Stato specifico della pagina di conversione.
/// Contiene tutti i dati necessari per la UI e la logica di conversione.
#[derive(Debug, Clone)]
pub struct ConvertPageState {
    pub input_path: String,
    pub output_format: String,
    pub output_dir: String,
    pub recursive: bool,
    pub quality: String,
    pub codec: String,
    pub jobs: String,
    pub available_formats: Vec<String>,
    pub selected_output_format: Option<String>,
}

impl Default for ConvertPageState {
    /// Implementazione del tratto `Default` per `ConvertPageState`.
    /// Fornisce i valori iniziali per lo stato della pagina.
    fn default() -> Self {
        Self {
            input_path: String::new(),
            output_format: "mp4".to_string(),
            output_dir: String::new(),
            recursive: false,
            quality: "192k".to_string(),
            codec: String::new(),
            jobs: "4".to_string(),
            available_formats: crate::list_formats(), // Assicurati che list_formats sia accessibile
            selected_output_format: Some("mp4".to_string()),
        }
    }
}
