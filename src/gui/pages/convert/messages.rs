//! Modulo per la gestione dei messaggi specifici della pagina di conversione.

use std::path::PathBuf;
use crate::gui::Message; // Importa Message dal modulo genitore

/// Enum per i messaggi specifici della pagina di conversione.
/// Ogni variante rappresenta un'azione o un evento che può verificarsi nella pagina.
#[derive(Debug, Clone)]
pub enum ConvertPageMessage {
    InputPathChanged(String),
    OutputDirChanged(String),
    RecursiveToggled(bool),
    QualityChanged(String),
    CodecChanged(String),
    JobsChanged(String),
    ButtonPressed,
    BrowseInput,
    BrowseOutput,
    InputFileSelected(Option<PathBuf>),
    OutputDirectorySelected(Option<PathBuf>),
    OutputFormatSelected(String),
    LoadAvailableFormats(Vec<String>),
}
