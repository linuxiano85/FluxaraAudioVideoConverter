//! Modulo per la visualizzazione della pagina di conversione.

use iced::{
    widget::{button, column, row, text_input, checkbox, PickList, text},
    Alignment, Element,
};
use crate::gui::Message; // Importa Message dal modulo genitore
use super::state::ConvertPageState; // Importa lo stato della pagina di conversione
use super::messages::ConvertPageMessage; // Importa i messaggi della pagina di conversione
use crate::i18n::translate;

/// Funzione per la visualizzazione della pagina di conversione.
/// Prende lo stato corrente e restituisce un `Element` che rappresenta l'interfaccia utente.
pub fn convert_page(state: &ConvertPageState) -> Element<'static, Message> {
    column![
        row![
            text_input(&translate("input_file_or_dir"), &state.input_path)
                .on_input(|s| Message::ConvertPage(ConvertPageMessage::InputPathChanged(s)))
                .padding(10),
            button(text(translate("browse_button"))).on_press(Message::ConvertPage(ConvertPageMessage::BrowseInput)),
        ]
        .spacing(10),
        row![
            text_input(&translate("output_dir_optional"), &state.output_dir)
                .on_input(|s| Message::ConvertPage(ConvertPageMessage::OutputDirChanged(s)))
                .padding(10),
            button(text(translate("browse_button"))).on_press(Message::ConvertPage(ConvertPageMessage::BrowseOutput)),
        ]
        .spacing(10),
        PickList::new(
            state.available_formats.clone(),
            state.selected_output_format.clone(),
            |s| Message::ConvertPage(ConvertPageMessage::OutputFormatSelected(s)),
        )
        .placeholder(&translate("choose_output_format"))
        .padding(10),
        checkbox(&translate("process_recursively"), state.recursive)
            .on_toggle(|b| Message::ConvertPage(ConvertPageMessage::RecursiveToggled(b))),
        text_input(&translate("audio_quality"), &state.quality)
            .on_input(|s| Message::ConvertPage(ConvertPageMessage::QualityChanged(s)))
            .padding(10),
        text_input(&translate("video_codec"), &state.codec)
            .on_input(|s| Message::ConvertPage(ConvertPageMessage::CodecChanged(s)))
            .padding(10),
        text_input(&translate("parallel_jobs"), &state.jobs)
            .on_input(|s| Message::ConvertPage(ConvertPageMessage::JobsChanged(s)))
            .padding(10),
        button(text(translate("start_conversion"))).on_press(Message::ConvertPage(ConvertPageMessage::ButtonPressed)),
    ]
    .align_items(Alignment::Center)
    .spacing(20)
    .padding(50)
    .into()
}
