//! Modulo per la gestione della logica di aggiornamento dello stato della pagina di conversione.

use iced::{Command};
use std::path::PathBuf;
use rfd::AsyncFileDialog; // Importa AsyncFileDialog

use crate::gui::Message; // Importa Message dal modulo genitore
use crate::convert_files; // Importa la funzione convert_files
use super::state::ConvertPageState; // Importa lo stato della pagina di conversione
use super::messages::ConvertPageMessage; // Importa i messaggi della pagina di conversione

/// Funzione per aggiornare lo stato della pagina di conversione.
/// Gestisce i messaggi specifici della pagina e aggiorna lo stato di conseguenza.
pub fn update(state: &mut ConvertPageState, message: ConvertPageMessage) -> iced::Command<Message> {
    match message {
        ConvertPageMessage::InputPathChanged(path) => {
            state.input_path = path;
        }
        ConvertPageMessage::OutputDirChanged(dir) => {
            state.output_dir = dir;
        }
        ConvertPageMessage::RecursiveToggled(toggle) => {
            state.recursive = toggle;
        }
        ConvertPageMessage::QualityChanged(quality) => {
            state.quality = quality;
        }
        ConvertPageMessage::CodecChanged(codec) => {
            state.codec = codec;
        }
        ConvertPageMessage::JobsChanged(jobs) => {
            state.jobs = jobs;
        }
        ConvertPageMessage::ButtonPressed => {
            println!("Convert button pressed!");
            let input_path = PathBuf::from(&state.input_path);
            let output_dir = if state.output_dir.is_empty() {
                None
            } else {
                Some(PathBuf::from(&state.output_dir))
            };
            let codec_option = if state.codec.is_empty() {
                None
            } else {
                Some(state.codec.clone())
            };
            let jobs_parsed = state.jobs.parse::<usize>().unwrap_or(4);

            let format_clone = state.output_format.clone();
            let quality_clone = state.quality.clone();
            let recursive_clone = state.recursive;
            return Command::perform(
                async move {
                    convert_files(
                        &input_path,
                        &format_clone,
                        output_dir.as_ref(),
                        recursive_clone,
                        &quality_clone,
                        codec_option.as_ref(),
                        jobs_parsed,
                    )
                    .await
                },
                |result| {
                    match result {
                        Ok(_) => {
                            println!("Conversion successful!");
                            Message::ConversionCompleted(Ok(()))
                        }
                        Err(e) => {
                            eprintln!("Conversion failed: {:?}", e);
                            Message::ConversionCompleted(Err(e.to_string()))
                        }
                    }
                },
            );
        }
        ConvertPageMessage::BrowseInput => {
            let current_path = state.input_path.clone();
            return Command::perform(
                async move {
                    AsyncFileDialog::new()
                        .set_directory(&current_path)
                        .pick_file()
                        .await
                },
                |file_handle| Message::ConvertPage(ConvertPageMessage::InputFileSelected(file_handle.map(|f| f.path().to_path_buf()))),
            );
        }
        ConvertPageMessage::BrowseOutput => {
            let current_path = state.output_dir.clone();
            return Command::perform(
                async move {
                    AsyncFileDialog::new()
                        .set_directory(&current_path)
                        .pick_folder()
                        .await
                },
                |file_handle| Message::ConvertPage(ConvertPageMessage::OutputDirectorySelected(file_handle.map(|f| f.path().to_path_buf()))),
            );
        }
        ConvertPageMessage::InputFileSelected(path) => {
            if let Some(path_buf) = path {
                state.input_path = path_buf.to_string_lossy().into_owned();
            }
        }
        ConvertPageMessage::OutputDirectorySelected(path) => {
            if let Some(path_buf) = path {
                state.output_dir = path_buf.to_string_lossy().into_owned();
            }
        }
        ConvertPageMessage::OutputFormatSelected(format) => {
            state.selected_output_format = Some(format.clone());
            state.output_format = format;
        }
        ConvertPageMessage::LoadAvailableFormats(formats) => {
            state.available_formats = formats;
            if let Some(format) = state.available_formats.first().cloned() {
                state.selected_output_format = Some(format.clone());
                state.output_format = format;
            }
        }
    }
    Command::none()
}
