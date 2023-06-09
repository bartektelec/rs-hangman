use iced::alignment;
use iced::widget::{
    self, button, column, container, row, text, Button, Column, Container, Row, Svg, Text,
};
use iced::{window, Alignment, Color, Element, Length, Sandbox, Settings};
use rand::prelude::*;

const GAME_OVER_STRIKE: i8 = 9;

struct Counter {
    value: i32,
    clicked: Vec<char>,
    target_word: String,
    tries: i8,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Inc,
    Dec,
    ClickChar(char),
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        let mut rng = rand::thread_rng();

        let file_output = std::fs::read_to_string("./assets/words.txt").unwrap();

        let words: Vec<&str> = file_output
            .split("\r\n")
            .collect::<Vec<&str>>()
            .into_iter()
            .filter(|w| w.len() > 0)
            .collect::<Vec<&str>>();

        let random_word = words[rng.gen_range(0..words.len())];

        Counter {
            value: 0,
            clicked: Vec::new(),
            target_word: random_word.to_string(),
            tries: 0,
        }
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
            Message::ClickChar(ch) => {
                self.clicked.push(ch);

                if !self.target_word.contains(ch) {
                    self.tries += 1;
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let censored_text = self
            .target_word
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .map(|ch| {
                if self.clicked.contains(ch) {
                    return ch;
                }

                &'_'
            })
            .collect::<String>();

        let game_over = self.target_word != censored_text && self.tries == GAME_OVER_STRIKE;
        let game_won = self.target_word == censored_text;

        let alphabet = [
            'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', 'a', 's', 'd', 'f', 'g', 'h', 'j',
            'k', 'l', 'z', 'x', 'c', 'v', 'b', 'n', 'm',
        ];

        let btn_row: Element<Message> = alphabet
            .iter()
            .fold(Row::new(), |container, ch| {
                let mut btn: Button<Message> = Button::new(Text::new(ch.to_string()));

                if !self.clicked.contains(ch) && !game_over {
                    btn = btn.on_press(Message::ClickChar(ch.clone()));
                }

                container.push(btn)
            })
            .into();

        let warning_text: Element<Message> = Text::new(if game_over {
            "You lost!".to_string()
        } else if game_won {
            "You won!".to_string()
        } else {
            format!("You have {} tries left.", GAME_OVER_STRIKE - self.tries)
        })
        .horizontal_alignment(alignment::Horizontal::Center)
        .style(if game_over {
            Color::from([1.0, 0.0, 0.0])
        } else if game_won {
            Color::from([0.0, 1.0, 0.0])
        } else {
            Color::from([0.0, 0.0, 0.0])
        })
        .into();

        let hints = Text::new(if game_over {
            self.target_word.clone()
        } else {
            censored_text
        })
        .size(30)
        .style(if game_won {
            Color::from([0.0, 1.0, 0.0])
        } else {
            Color::from([0.0, 0.0, 0.0])
        })
        .width(Length::Fill)
        .vertical_alignment(alignment::Vertical::Center)
        .horizontal_alignment(alignment::Horizontal::Center);

        let image = Svg::from_path(format!("./assets/{}.svg", &self.tries.to_string()));
        let image_container = container(image).center_x().center_y();

        let app = column![warning_text, hints, image_container, btn_row].spacing(30);

        container(app)
            .width(Length::Fill)
            .padding(40)
            .center_x()
            .into()
    }
}

pub fn main() -> iced::Result {
    Counter::run(Settings {
        window: window::Settings {
            size: (640, 640),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}
