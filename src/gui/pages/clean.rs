use iced::{
    widget::{button, column, row, text, text_input, checkbox},
    Alignment,
};
use crate::gui::Message; // Importa Message dal modulo genitore

pub fn clean_page(
    input: &str,
    metadata: bool,
    optimize: bool,
    recursive: bool,
) -> iced::widget::Column<'static, Message> {
    column![
        text("Clean Media Files").size(30),
        row![
            text_input("Input File or Directory", input)
                .on_input(Message::CleanInputChanged)
                .padding(10),
            button("Browse").on_press(Message::CleanBrowseInput),
        ]
        .spacing(10),
        checkbox("Remove Metadata", metadata)
            .on_toggle(Message::CleanMetadataToggled),
        checkbox("Optimize File Size", optimize)
            .on_toggle(Message::CleanOptimizeToggled),
        checkbox("Process Recursively", recursive)
            .on_toggle(Message::CleanRecursiveToggled),
        button("Start Cleaning").on_press(Message::CleanButtonPressed),
    ]
    .align_items(Alignment::Center)
    .spacing(20)
    .padding(50)
}
