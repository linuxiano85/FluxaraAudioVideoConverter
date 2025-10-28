//! Modulo principale per l'interfaccia utente grafica (GUI) dell'applicazione.
//!
//! Questo modulo definisce la struttura principale dell'applicazione Iced,
//! gestendo lo stato, gli aggiornamenti e la visualizzazione delle diverse pagine.
//!
//! Contiene i seguenti sottomoduli:
//! - `pages`: Definisce le diverse pagine dell'applicazione.
//! - `theme`: Gestisce il tema visivo dell'applicazione.
//! - `state`: Contiene la definizione dello stato globale dell'applicazione e delle pagine.
//! - `update`: Gestisce la logica di aggiornamento dello stato in risposta ai messaggi.
//! - `view`: Gestisce la logica di rendering dell'interfaccia utente.

pub mod pages;
pub mod theme;
pub mod state;
pub mod update;
pub mod view;

use iced::{Application, Settings, Command, Subscription, Element, Theme, Renderer};
use crate::i18n::translate; // Removed `self`
use state::{App, Page};
use theme::AppTheme;

/// Punto di ingresso principale per l'applicazione GUI.
/// Avvia l'applicazione Iced.
///
/// # Restituisce
///
/// Un `iced::Result` che indica il successo o il fallimento dell'avvio dell'applicazione.
pub fn run() -> iced::Result {
    App::run(Settings::default())
}

/// Enum che definisce tutti i messaggi che l'applicazione può elaborare.
/// Questi messaggi vengono inviati all'applicazione per aggiornare il suo stato.
///
/// I messaggi sono suddivisi per pagina o per funzionalità globale.
#[derive(Debug, Clone)]
pub enum Message {
    PageChanged(Page),
    // Messaggi specifici per la pagina di conversione
    ConvertPage(pages::convert::messages::ConvertPageMessage),
    // Messaggi per la pagina di miglioramento audio
    EnhanceAudioInputChanged(String),
    EnhanceAudioOutputChanged(String),
    EnhanceAudioDenoiseToggled(bool),
    EnhanceAudioNormalizeToggled(bool),
    EnhanceAudioHighpassChanged(String),
    EnhanceAudioLowpassChanged(String),
    EnhanceAudioNotchChanged(String),
    EnhanceAudioCompressorToggled(bool),
    EnhanceAudioGateToggled(bool),
    EnhanceAudioGateThresholdChanged(String),
    EnhanceAudioOnlyToggled(bool),
    EnhanceAudioButtonPressed,
    EnhanceAudioBrowseInput,
    EnhanceAudioBrowseOutput,
    EnhanceAudioInputFileSelected(Option<std::path::PathBuf>),
    EnhanceAudioOutputFileSelected(Option<std::path::PathBuf>),
    // Messaggi per la pagina di miglioramento video
    EnhanceVideoInputChanged(String),
    EnhanceVideoOutputChanged(String),
    EnhanceVideoDeinterlaceToggled(bool),
    EnhanceVideoStabilizeToggled(bool),
    EnhanceVideoDenoiseTypeChanged(String),
    EnhanceVideoSharpenToggled(bool),
    EnhanceVideoColorToggled(bool),
    EnhanceVideoWidthChanged(String),
    EnhanceVideoHeightChanged(String),
    EnhanceVideoAspectChanged(String),
    EnhanceVideoButtonPressed,
    EnhanceVideoBrowseInput,
    EnhanceVideoBrowseOutput,
    EnhanceVideoInputFileSelected(Option<std::path::PathBuf>),
    EnhanceVideoOutputFileSelected(Option<std::path::PathBuf>),
    // Messaggi per la pagina di recupero VHS
    VhsRescueInputChanged(String),
    VhsRescueOutputChanged(String),
    VhsRescueNotchChanged(String),
    VhsRescueButtonPressed,
    VhsRescueBrowseInput,
    VhsRescueBrowseOutput,
    VhsRescueInputFileSelected(Option<std::path::PathBuf>),
    VhsRescueOutputFileSelected(Option<std::path::PathBuf>),
    // Messaggi per la pagina di acquisizione
    CaptureOutputChanged(String),
    CaptureVideoDeviceChanged(String),
    CaptureAudioDeviceChanged(String),
    CaptureFormatChanged(String),
    CaptureDeinterlaceToggled(bool),
    CaptureStabilizeToggled(bool),
    CaptureDenoiseChanged(String),
    CaptureVbitrateChanged(String),
    CaptureCrfChanged(String),
    CaptureWidthChanged(String),
    CaptureHeightChanged(String),
    CaptureFpsChanged(String),
    CaptureAbitrateChanged(String),
    CaptureArchivalToggled(bool),
    CaptureButtonPressed,
    CaptureBrowseOutput,
    CaptureOutputFileSelected(Option<std::path::PathBuf>),
    VideoDevicesLoaded(Vec<String>),
    AudioDevicesLoaded(Vec<String>),
    // Messaggi per la pagina di pulizia
    CleanInputChanged(String),
    CleanMetadataToggled(bool),
    CleanOptimizeToggled(bool),
    CleanRecursiveToggled(bool),
    CleanButtonPressed,
    CleanBrowseInput,
    CleanInputFileSelected(Option<std::path::PathBuf>),
    // Messaggi per la pagina informazioni
    InfoInputChanged(String),
    InfoButtonPressed,
    InfoBrowseInput,
    InfoInputFileSelected(Option<std::path::PathBuf>),
    // Messaggi per la pagina formati
    FormatsButtonPressed,
    // Messaggi globali di stato e tema
    ConversionStarted,
    ConversionProgressed(f32),
    ConversionCompleted(Result<(), String>),
    UpdateStatus(String),
    ThemeChanged(AppTheme),
    LanguageChanged(crate::i18n::Language),
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    /// Inizializza lo stato dell'applicazione.
    ///
    /// Questa funzione viene chiamata all'avvio dell'applicazione per creare lo stato iniziale
    /// e per eseguire comandi iniziali, come il caricamento dei dispositivi audio/video.
    ///
    /// # Argomenti
    ///
    /// * `_flags` - Flag di inizializzazione (non utilizzati in questa applicazione).
    ///
    /// # Restituisce
    ///
    /// Una tupla contenente lo stato iniziale dell'applicazione e un `Command`
    /// per eseguire operazioni asincrone.
    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            App::default(),
            Command::batch(vec![
                Command::perform(async { crate::capture::list_video_devices().unwrap_or_default() }, Message::VideoDevicesLoaded),
                Command::perform(async { crate::capture::list_audio_devices().unwrap_or_default() }, Message::AudioDevicesLoaded),
            ]),
        )
    }

    /// Restituisce il titolo della finestra dell'applicazione.
    ///
    /// Il titolo viene tradotto in base alla lingua corrente e alla pagina visualizzata.
    ///
    /// # Restituisce
    ///
    /// Una `String` che rappresenta il titolo della finestra.
    fn title(&self) -> String {
        translate(&format!("title_{:?}", self.current_page))
    }

    /// Aggiorna lo stato dell'applicazione in base al messaggio ricevuto.
    ///
    /// Questa funzione delega l'aggiornamento dello stato al modulo `update`.
    ///
    /// # Argomenti
    ///
    /// * `message` - Il messaggio da elaborare.
    ///
    /// # Restituisce
    ///
    /// Un `Command<Message>` che può essere `Command::none()` o un comando per
    /// eseguire effetti collaterali.
    fn update(&mut self, message: Message) -> Command<Message> {
        update::update(self, message)
    }

    /// Costruisce l'interfaccia utente dell'applicazione.
    ///
    /// Questa funzione delega la costruzione dell'interfaccia utente al modulo `view`.
    ///
    /// # Restituisce
    ///
    /// Un `Element` che rappresenta l'interfaccia utente dell'applicazione.
    fn view(&self) -> Element<'_, Message, Theme, Renderer> {
        view::view(self)
    }

    /// Restituisce il tema corrente dell'applicazione.
    ///
    /// Il tema viene derivato dal `current_theme` memorizzato nello stato dell'applicazione.
    ///
    /// # Restituisce
    ///
    /// Il `Theme` corrente dell'applicazione.
    fn theme(&self) -> Theme {
        Theme::from(self.current_theme)
    }

    /// Gestisce le sottoscrizioni per eventi esterni.
    ///
    /// Attualmente non ci sono sottoscrizioni attive, quindi restituisce `Subscription::none()`.
    ///
    /// # Restituisce
    ///
    /// Un `Subscription<Message>` per gestire gli eventi esterni.
    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}
