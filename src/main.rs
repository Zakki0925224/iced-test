use iced::{button, Sandbox, Element, Button, Text, Result, Settings};

fn main() -> Result
{
    return App::run(Settings::default());
}

#[derive(Default)]
struct App
{
    button_state: button::State
}

#[derive(Debug, Clone)]
enum Message
{
    ButtonPressed
}

impl Sandbox for App
{
    type Message = Message;

    fn new() -> Self
    {
        return Self::default();
    }

    fn title(&self) -> String
    {
        return "My App".to_string();
    }

    fn update(&mut self, message: Message)
    {
        match message
        {
            Message::ButtonPressed => println!("Button pressed!")
        }
    }

    fn view(&mut self) -> Element<Message>
    {
        return Button::new(&mut self.button_state, Text::new("Button"))
            .on_press(Message::ButtonPressed)
            .into();
    }
}