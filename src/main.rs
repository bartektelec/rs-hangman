use iced::widget::{button, column, container, text, Column, Container};
use iced::{Element, Sandbox, Settings};

struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Inc,
    Dec,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Counter { value: 0 }
    }

    fn title(&self) -> String {
        String::from("Hangman Game")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Inc => {
                self.value += 1;
            }
            Message::Dec => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            button("+").on_press(Message::Inc),
            text(self.value).size(50),
            button("-").on_press(Message::Dec),
        ]
        .spacing(20)
        .max_width(800);

        container(content).width(800).padding(40).center_x().into()
    }
}

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}
