use iced::{
    widget::{button, column, row, text, Space},
    Alignment, Element, Length, Theme, Renderer, Color,
};
use crate::i18n::translate;

use crate::gui::state::{App, Page};
use crate::gui::Message;
use crate::gui::theme::AppTheme;
use crate::gui::pages::convert::view as convert_view;
use crate::gui::pages;

/// Crea un pulsante di navigazione per la sidebar.
/// Il pulsante sarà evidenziato se la pagina corrispondente è quella corrente.
///
/// # Argomenti
///
/// * `page` - La pagina a cui il pulsante naviga.
/// * `current_page` - La pagina attualmente visualizzata.
/// * `app_theme` - Il tema corrente dell'applicazione.
///
/// # Restituisce
///
/// Un `Element` che rappresenta il pulsante di navigazione.
fn nav_button(page: Page, current_page: Page, app_theme: &AppTheme) -> Element<'static, Message, Theme, Renderer> {
    let is_active = page == current_page;
    let button_style = if is_active {
        iced::theme::Button::Primary
    } else {
        iced::theme::Button::Secondary
    };

    button(text(translate(&format!("nav_{}", page.to_string().to_lowercase())))
        .size(20)
        .horizontal_alignment(iced::alignment::Horizontal::Center)
        .style(iced::theme::Text::Color((*app_theme).into())))
        .on_press(Message::PageChanged(page))
        .padding(10)
        .width(Length::Fill)
        .style(app_theme) // Pass app_theme directly
        .into()
}

/// Costruisce l'interfaccia utente dell'applicazione.
///
/// Questa funzione prende lo stato corrente dell'applicazione e restituisce un `Element`
/// che rappresenta l'intera interfaccia utente. Include la sidebar di navigazione,
/// il contenuto della pagina corrente e una barra di stato.
///
/// # Argomenti
///
/// * `app` - Un riferimento allo stato corrente dell'applicazione.
///
/// # Restituisce
///
/// Un `Element` che rappresenta l'interfaccia utente dell'applicazione.
pub fn view(app: &App) -> Element<'_, Message, Theme, Renderer> {
    let sidebar = column![
        nav_button(Page::Home, app.current_page, &app.current_theme),
        nav_button(Page::Convert, app.current_page, &app.current_theme),
        nav_button(Page::EnhanceAudio, app.current_page, &app.current_theme),
        nav_button(Page::EnhanceVideo, app.current_page, &app.current_theme),
        nav_button(Page::VhsRescue, app.current_page, &app.current_theme),
        nav_button(Page::Capture, app.current_page, &app.current_theme),
        nav_button(Page::Clean, app.current_page, &app.current_theme),
        nav_button(Page::Info, app.current_page, &app.current_theme),
        nav_button(Page::Formats, app.current_page, &app.current_theme),
        nav_button(Page::Settings, app.current_page, &app.current_theme),
    ]
    .spacing(10)
    .width(Length::Fixed(200.0))
    .padding(20);

    let content_widget: Element<'_, Message, Theme, Renderer> = match app.current_page {
        Page::Home => pages::home::home_page().into(),
        Page::Convert => convert_view::convert_page(
            &app.convert_page_state,
        ).into(),
        Page::EnhanceAudio => pages::enhance_audio::enhance_audio_page(
            &app.enhance_audio_input,
            &app.enhance_audio_output,
            app.enhance_audio_denoise,
            app.enhance_audio_normalize,
            &app.enhance_audio_highpass,
            &app.enhance_audio_lowpass,
            &app.enhance_audio_notch,
            app.enhance_audio_compressor,
            app.enhance_audio_gate,
            &app.enhance_audio_gate_threshold,
            app.enhance_audio_only,
        ).into(),
        Page::EnhanceVideo => pages::enhance_video::enhance_video_page(
            &app.enhance_video_input,
            &app.enhance_video_output,
            app.enhance_video_deinterlace,
            app.enhance_video_stabilize,
            &app.enhance_video_denoise_type,
            app.enhance_video_sharpen,
            app.enhance_video_color,
            &app.enhance_video_width,
            &app.enhance_video_height,
            &app.enhance_video_aspect,
        ).into(),
        Page::VhsRescue => pages::vhs_rescue::vhs_rescue_page(
            &app.vhs_rescue_input,
            &app.vhs_rescue_output,
            &app.vhs_rescue_notch,
            app.vhs_rescue_deinterlace,
            app.vhs_rescue_stabilize,
            &app.vhs_rescue_denoise_type,
            app.vhs_rescue_sharpen,
            app.vhs_rescue_color,
            &app.vhs_rescue_highpass,
            &app.vhs_rescue_lowpass,
            app.vhs_rescue_compressor,
            app.vhs_rescue_gate,
            app.vhs_rescue_normalize,
        ).into(),
        Page::Capture => pages::capture::capture_page(
            &app.capture_output,
            &app.capture_video_device,
            &app.video_devices,
            &app.capture_audio_device,
            &app.audio_devices,
            &app.capture_format,
            app.capture_deinterlace,
            app.capture_stabilize,
            &app.capture_denoise,
            &app.capture_vbitrate,
            &app.capture_crf,
            &app.capture_width,
            &app.capture_height,
            &app.capture_fps,
            &app.capture_abitrate,
            app.capture_archival,
        ).into(),
        Page::Clean => pages::clean::clean_page(
            &app.clean_input,
            app.clean_metadata,
            app.clean_optimize,
            app.clean_recursive,
        ).into(),
        Page::Info => pages::info::info_page(
            &app.info_input,
        ).into(),
        Page::Formats => pages::formats::formats_page().into(),
        Page::Settings => pages::settings::settings_page(app.current_theme, app.current_language).into(),
    };

    let content: Element<Message, Theme, Renderer> = content_widget.into();

    let status_bar: Element<'_, Message, Theme, Renderer> = row![
        text::<'_, Theme, Renderer>(&app.status_message).size(16).style(iced::theme::Text::Color(app.current_theme.into())),
        Into::<Element<'_, Message, Theme, Renderer>>::into(
            Space::with_width(Length::Fixed(200.0))
        ),
        if let Some(error) = &app.error_message {
            text::<'_, Theme, Renderer>(error).size(16).style(iced::theme::Text::Color(Color::from_rgb(1.0, 0.0, 0.0))).into()
        } else {
            Into::<Element<'_, Message, Theme, Renderer>>::into(
                Space::with_width(Length::Fixed(0.0))
            )
        }
    ]
    .spacing(10)
    .align_items(Alignment::Center)
    .padding(10)
    .width(Length::Fill)
    .into();

    column![
        row![sidebar, content]
            .spacing(20)
            .align_items(Alignment::Start)
            .height(Length::Fill),
        status_bar,
    ]
    .into()
}
