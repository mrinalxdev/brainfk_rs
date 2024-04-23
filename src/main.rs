use iced::{widget::text, Element, Sandbox, Settings};

fn main() -> iced::Result {
    Editor::run(Settings::default())
}

struct Editor;

#[derive(Debug)]
enum Message {}

impl Sandbox for Editor {
    type Message = Message;

    fn new() -> Self {
        Self
        
    }

    fn title(&self) -> String {
        String::from("A cool editor")
    }
    
    fn update(&mut self, message: Message) {
        match message {}
    }
    
    fn view(&self) -> Element<'_, Message> {
        text("Hello, Iced").into()
    }

}