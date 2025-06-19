use iced::widget::{column, horizontal_space, row, text_editor, toggler};
use iced::{Element, Fill, Font, Theme};

pub static PREETI_FONT: &[u8] = include_bytes!("../fonts/Preeti Normal.otf");

fn main() -> iced::Result {
    iced::application("Unicode to Preeti", Uni2Preeti::update, Uni2Preeti::view)
        .theme(Uni2Preeti::theme)
        .font(PREETI_FONT)
        .run()
}

#[derive(Default)]
pub struct Uni2Preeti {
    pub dark_theme: bool,
    unicode: text_editor::Content,
    preeti: text_editor::Content,
}

#[derive(Debug, Clone)]
pub enum Message {
    ThemeChange(bool),
    Unicode(text_editor::Action),
    Preeti(text_editor::Action),
}

impl Uni2Preeti {
    fn update(&mut self, message: Message) {
        match message {
            Message::ThemeChange(theme) => {
                self.dark_theme = theme;
            }
            Message::Unicode(action) => {
                self.unicode.perform(action);
            }
            Message::Preeti(action) => {
                self.preeti.perform(action);
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let controls = row![
            horizontal_space(),
            toggler(self.dark_theme).on_toggle(Message::ThemeChange)
        ]
        .spacing(10)
        .padding(10);

        let unicode = text_editor(&self.unicode)
            .height(Fill)
            .size(20)
            .on_action(Message::Unicode);
        let preeti = text_editor(&self.preeti)
            .height(Fill)
            .font(Font::with_name("Preeti"))
            .size(25)
            .on_action(Message::Preeti);
        column![controls, row![unicode, preeti].spacing(20)]
            .spacing(20)
            .padding(25)
            .into()
    }

    fn theme(&self) -> Theme {
        if self.dark_theme {
            Theme::Dark
        } else {
            Theme::Light
        }
    }
}
