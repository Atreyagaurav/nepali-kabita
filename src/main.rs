use iced::widget::{button, center, column, horizontal_space, row, slider, text_editor, toggler};
use iced::{Element, Fill, Font, Theme};

pub static PREETI_FONT: &[u8] = include_bytes!("../fonts/Preeti Normal.otf");

fn main() -> iced::Result {
    iced::application("Unicode to Preeti", Uni2Preeti::update, Uni2Preeti::view)
        .theme(Uni2Preeti::theme)
        .font(PREETI_FONT)
        .run()
}

pub struct Uni2Preeti {
    dark_theme: bool,
    show_preeti: bool,
    uni_size: u16,
    pre_size: u16,
    unicode: text_editor::Content,
    preeti: text_editor::Content,
}

impl Default for Uni2Preeti {
    fn default() -> Self {
        Self {
            dark_theme: true,
            show_preeti: false,
            uni_size: 20,
            pre_size: 25,
            unicode: text_editor::Content::default(),
            preeti: text_editor::Content::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ThemeChange(bool),
    TogglePreeti(bool),
    UniFontSize(u16),
    PreFontSize(u16),
    Unicode(text_editor::Action),
    Preeti(text_editor::Action),
}

impl Uni2Preeti {
    fn update(&mut self, message: Message) {
        match message {
            Message::ThemeChange(theme) => {
                self.dark_theme = theme;
            }
            Message::TogglePreeti(v) => self.show_preeti = v,
            Message::UniFontSize(s) => self.uni_size = s,
            Message::PreFontSize(s) => self.pre_size = s,
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

        let unicode = column![
            text_editor(&self.unicode)
                .height(Fill)
                .size(self.uni_size)
                .placeholder("Type in Unicode")
                .on_action(Message::Unicode),
            slider(8..=100, self.uni_size, Message::UniFontSize).width(Fill)
        ]
        .spacing(5);
        let preeti: Element<_> = if self.show_preeti {
            row![
                column![
                    text_editor(&self.preeti)
                        .height(Fill)
                        .font(Font::with_name("Preeti"))
                        .size(self.pre_size)
                        .placeholder("Type in Preeti")
                        .on_action(Message::Preeti),
                    slider(8..=100, self.pre_size, Message::PreFontSize).width(Fill)
                ]
                .spacing(5),
                button(center(">"))
                    .on_press(Message::TogglePreeti(false))
                    .height(Fill)
                    .width(25)
            ]
            .into()
        } else {
            button(center("<"))
                .on_press(Message::TogglePreeti(true))
                .height(Fill)
                .width(25)
                .into()
        };
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
