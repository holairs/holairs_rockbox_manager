use iced::widget::{container, text};
use iced::{Element, Length};

#[derive(Debug, Clone, Copy)]
pub enum Message {}

pub fn view(_state: &()) -> Element<Message> {
    let title = text("Playlist Manager").size(40); // Font size

    // El contenedor centra el texto en toda la pantalla
    container(title)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(100)
        .center_y(100)
        .into()
}
