use iced::widget::{button, column, text, text_input, pick_list};
use iced::{Alignment, Element, Sandbox, Settings};


pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

struct Counter {
    input: String,
    from: Option<Temperature>,
    to: Option<Temperature>,
    result: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Temperature {
    Fahrenheit,
    Celsius,
    Kelvin,
    Other,
}

impl Temperature {
    const ALL: [Temperature; 4] = [
        Temperature::Fahrenheit,
        Temperature::Celsius,
        Temperature::Kelvin,
        Temperature::Other,
    ];
}

impl std::fmt::Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Temperature::Fahrenheit => "Fahrenheit",
                Temperature::Celsius => "Celsius",
                Temperature::Kelvin => "Kelvin",
                Temperature::Other => "Other",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeTextInput(String),
    FromSelected(Temperature),
    ToSelected(Temperature),
    Calculate,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self { 
            input: "".into(), 
            from: Some(Temperature::Other), 
            to: Some(Temperature::Other), 
            result: 0 
        }
    }

    fn title(&self) -> String {
        String::from("Temperature Converter")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ChangeTextInput(value) => {
                self.input = value.into();
            }
            Message::FromSelected(temperature) => {
                self.from = Some(temperature);
            }
            Message::ToSelected(temperature) => {
                self.to = Some(temperature);
            }
            Message::Calculate => {
                let result = self.input
                            .parse::<i32>()
                            .expect("Input is not a number");

                let result = match (self.from, self.to) {
                    (Some(Temperature::Celsius), Some(Temperature::Fahrenheit)) 
                        => (result * 9/5) + 32,
                    (Some(Temperature::Celsius), Some(Temperature::Kelvin)) 
                        => result + 273,
                    (Some(Temperature::Fahrenheit), Some(Temperature::Celsius)) 
                        => (result - 32) * 5/9,
                    (Some(Temperature::Fahrenheit), Some(Temperature::Kelvin)) 
                        => (result - 32) * 5 / 9 + 273,
                    (Some(Temperature::Kelvin), Some(Temperature::Celsius)) 
                        => result - 273,
                    (Some(Temperature::Kelvin), Some(Temperature::Fahrenheit)) 
                        => 2 * (result - 273) + 32,
                    _ => 0,
                };

                self.result = result;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            text_input("Temperatuur calculate", &self.input).on_input(Message::ChangeTextInput),
            text("From : ").size(25),
            pick_list(
                &Temperature::ALL[..],
                self.from,
                Message::FromSelected,
            ),

            text("To : ").size(25),
            pick_list(
                &Temperature::ALL[..],
                self.to,
                Message::ToSelected,
            ),
            button("Calculate").on_press(Message::Calculate),
            text("Result : ").size(25),
            text(self.result).size(25),
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }

}
