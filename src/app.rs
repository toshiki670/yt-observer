use iced::{
    executor, mouse, theme,
    widget::{canvas, column, container, horizontal_space, pick_list, row, scrollable, text},
    Alignment, Application, Command, Element, Font, Length, Point, Rectangle, Renderer, Theme,
};

#[derive(Default, Debug)]
pub struct App {
    theme: Theme,
}

#[derive(Debug, Clone)]
pub enum Message {
    ThemeSelected(Theme),
}

impl Application for App {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let app = Self { theme: Theme::Dark };
        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("Youtube observer")
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::ThemeSelected(theme) => {
                self.theme = theme;
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let header = container(
            row![
                text("Youtube Observer").size(20).font(Font::MONOSPACE),
                horizontal_space(),
                pick_list(Theme::ALL, Some(&self.theme), Message::ThemeSelected),
            ]
            .spacing(20)
            .align_items(Alignment::Center),
        )
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();

            container::Appearance::default().with_border(palette.background.strong.color, 1)
        });

        let sidebar = container(
            column!["Sidebar!", square(50), square(50)]
                .spacing(40)
                .padding(10)
                .width(200)
                .align_items(Alignment::Center),
        )
        .style(theme::Container::Box)
        .height(Length::Fill)
        .center_y();

        let content = container(
            scrollable(
                column!["Content!", square(400), square(200), square(400), "The end"]
                    .spacing(40)
                    .align_items(Alignment::Center)
                    .width(Length::Fill),
            )
            .height(Length::Fill),
        )
        .padding(10);

        column![header, row![sidebar, content]].into()
    }
}

fn square<'a>(size: impl Into<Length> + Copy) -> Element<'a, Message> {
    struct Square;

    impl canvas::Program<Message> for Square {
        type State = ();

        fn draw(
            &self,
            _state: &Self::State,
            renderer: &Renderer,
            theme: &Theme,
            bounds: Rectangle,
            _cursor: mouse::Cursor,
        ) -> Vec<canvas::Geometry> {
            let mut frame = canvas::Frame::new(renderer, bounds.size());

            let palette = theme.extended_palette();

            frame.fill_rectangle(
                Point::ORIGIN,
                bounds.size(),
                palette.background.strong.color,
            );

            vec![frame.into_geometry()]
        }
    }

    canvas(Square).width(size).height(size).into()
}
