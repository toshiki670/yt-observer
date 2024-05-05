use iced::{
    executor, mouse,
    widget::{canvas, column, container, row, scrollable},
    Alignment, Application, Command, Element, Length, Point, Rectangle, Renderer, Theme,
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

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Youtube observer")
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
        let sidebar = container(
            column!["Sidebar!", square(50), square(50)]
                .spacing(40)
                .padding(10)
                .width(200)
                .align_items(Alignment::Center),
        )
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

        row![sidebar, content].into()
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
