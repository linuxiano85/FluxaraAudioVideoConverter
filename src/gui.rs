use iced::{
    widget::{button, column, text},
    Element, Sandbox, Settings,
};

pub fn run() -> iced::Result {
    App::run(Settings::default())
}

struct App;

#[derive(Debug, Clone, Copy)]
enum Message {
    ButtonPressed,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        App
    }

    fn title(&self) -> String {
        String::from("Fluxara AVC")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => {
                println!("Button pressed!");
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        column![
            text("Welcome to Fluxara AVC GUI!"),
            button("Click me!").on_press(Message::ButtonPressed),
        ]
        .into()
    }
}
