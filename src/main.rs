mod ui;

use iced::font::{self, Font};
use iced::widget::{
    self, canvas, column, container, horizontal_space, row, vertical_space, Column, Space,
};
use iced::window::{self, Position};
use iced::{color, executor, keyboard, mouse, Color, Pixels, Point, Rectangle, Renderer};
use iced::{Application, Command, Element, Length, Settings, Theme};
use iced::{Size, Subscription};
use ui::UI;

const FONT: Font = Font::with_name("Inter");

pub fn main() -> iced::Result {
    let target_size = Size {
        width: 1280_f32,
        height: 832_f32,
    };

    Example::run(Settings {
        window: window::Settings {
            min_size: Some(target_size),
            size: target_size,
            position: Position::Specific(Point::new(3500.0, 300.0)), // todo Position::Centered
            ..window::Settings::default()
        },
        default_text_size: Pixels(13.0),
        // antialiasing: true,
        default_font: FONT,
        ..Settings::default()
    })
}

struct Connection {
    name: String,
    color: Color,
    connection_type: Option<Database>,
    host: String,
    port: String,
    user: String,
    password: String,
    default_database: String,
}

impl Default for Connection {
    fn default() -> Self {
        Connection {
            name: String::from("My Acme Database"),
            color: color!(0xE7298A),
            connection_type: Some(Database::Postgres),
            host: String::from("localhost"),
            port: String::from("5432"),
            user: String::from("root"),
            password: String::from("password"),
            default_database: String::from("acme"),
        }
    }
}

#[derive(Default)]
struct Example {
    explain: bool,

    search_input: String,
    connection: Connection,
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
    FontLoaded(Result<(), font::Error>),

    TabPressed { shift: bool },

    SearchInputChanged(String),
    // connections
    NameInputChanged(String),
    ConnectionTypeSelected(Database),
    HostInputChanged(String),
    PortInputChanged(String),
    UserInputChanged(String),
    PasswordInputChanged(String),
    DefaultDatabaseInputChanged(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Database {
    Postgres,
    Mariadb,
}

impl Default for Database {
    fn default() -> Self {
        Database::Postgres
    }
}

impl Database {
    const ALL: [Database; 2] = [Database::Postgres, Database::Mariadb];
}

impl std::fmt::Display for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Database::Postgres => "Postgres",
                Database::Mariadb => "Mariadb",
            }
        )
    }
}

impl Application for Example {
    type Message = Message;
    type Flags = ();
    type Executor = executor::Default;
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Example {
                connection: Connection {
                    ..Connection::default()
                },
                ..Self::default()
            },
            font::load(include_bytes!("../fonts/Inter-VariableFont_slnt,wght.ttf").as_slice())
                .map(Message::FontLoaded),
        )
    }

    fn title(&self) -> String {
        String::from("Iced - ui - test")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ButtonPressed => {
                self.explain = !self.explain;
            }
            Message::NameInputChanged(value) => {
                self.connection.name = value;
            }
            Message::ConnectionTypeSelected(connection_type) => {
                self.connection.connection_type = Some(connection_type);
            }
            Message::HostInputChanged(value) => {
                self.connection.host = value;
            }
            Message::PortInputChanged(value) => {
                self.connection.port = value; // todo need to validate to only allow numbers / or move this into a digit input
            }
            Message::UserInputChanged(value) => {
                self.connection.user = value;
            }
            Message::PasswordInputChanged(value) => {
                self.connection.password = value;
            }
            Message::DefaultDatabaseInputChanged(value) => {
                self.connection.default_database = value;
            }
            Message::FontLoaded(_) => (),
            Message::TabPressed { shift } => {
                if shift {
                    return widget::focus_previous();
                } else {
                    return widget::focus_next();
                }
            }
            Message::SearchInputChanged(value) => self.search_input = value,
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        use keyboard::key;

        keyboard::on_key_press(|key, modifiers| {
            let keyboard::Key::Named(key) = key else {
                return None;
            };

            match (key, modifiers) {
                (key::Named::Tab, _) => Some(Message::TabPressed {
                    shift: modifiers.shift(),
                }),
                _ => None,
            }
        })
    }

    // todo look into pane grid instead of containers

    fn view(&self) -> Element<Message> {
        let ui = UI {};

        let main_content = self.connection_main_content(ui);
        let left_content = self.left_content(ui);

        let left = container(left_content)
            // .max_width(300) // this doesn't work with width full
            // .width(Length::Fill)
            .width(300)
            .height(Length::Fill);
        let main = container(main_content)
            .style(
                container::Appearance::default().with_background(ui::UI::COLOR_STYLES.base.base20),
            )
            .padding(8)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x();

        let screen: Element<_> = container(row![left, main])
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(
                container::Appearance::default().with_background(ui::UI::COLOR_STYLES.base.base19),
            )
            .into();

        container(if self.explain {
            screen.explain(color! {0x0000ff})
        } else {
            screen
        })
        .into()
    }
}

impl UI {
    fn select_item<'a>(
        &self,
        text: &str,
        item_ele: Element<'a, Message>,
        level: u16,
        selected: bool,
    ) -> Element<'a, Message> {
        let item = row![item_ele, self.text(text)]
            .padding([2, 8, 2, 16 * (level - 1) + 8 * level]) // todo  should be 4,8 need text size adjust
            .height(24)
            .width(Length::Fill)
            .spacing(12)
            .align_items(iced::Alignment::Center);
        if !selected {
            return item.into();
        }

        container(item)
            .style(
                container::Appearance::default()
                    .with_background(ui::UI::COLOR_STYLES.accent.color9)
                    .with_border(ui::UI::COLOR_STYLES.accent.color5, 1),
            )
            .into()
    }
}

impl Example {
    fn connection_main_content(&self, ui: UI) -> widget::Container<'_, Message> {
        let one = row![
            column![
                ui.label("Name"),
                ui.text_input("", &self.connection.name)
                    .on_input(Message::NameInputChanged)
            ]
            .spacing(4),
            fake_color_picker(self.connection.color)
        ]
        .align_items(iced::Alignment::End)
        .spacing(8);

        let two = column![
            ui.label("Connection Type"),
            ui.select(
                &Database::ALL[..],
                self.connection.connection_type,
                Message::ConnectionTypeSelected,
            )
        ]
        .spacing(4);

        let three = row![
            column![
                ui.label("Host"),
                ui.text_input("", &self.connection.host)
                    .on_input(Message::HostInputChanged)
            ]
            .spacing(4),
            column![
                ui.label("Port"),
                ui.text_input("", &self.connection.port)
                    .on_input(Message::PortInputChanged)
            ]
            .width(80)
            .spacing(4)
        ]
        .spacing(8);

        let four = row![
            column![
                ui.label("User"),
                ui.text_input("", &self.connection.user)
                    .on_input(Message::UserInputChanged)
            ]
            .spacing(4),
            column![
                ui.label("Password"),
                ui.text_input("", &self.connection.password)
                    .on_input(Message::PasswordInputChanged)
                    .secure(true) // todo secure should not allow copy paste
            ]
            .spacing(4)
        ]
        .spacing(8);

        let five = column![
            ui.label("Default Database"),
            ui.text_input("", &self.connection.default_database)
                .on_input(Message::DefaultDatabaseInputChanged)
        ]
        .spacing(4);

        let six = row![
            horizontal_space(),
            ui.button_secondary("Save").on_press(Message::ButtonPressed),
            ui.button_secondary("Test").on_press(Message::ButtonPressed),
            ui.button("Save & Connect").on_press(Message::ButtonPressed),
        ]
        .spacing(8)
        .width(Length::Fill);

        let main_content = container(
            column![
                column![ui.heading("Connection")].padding([0, 0, 8, 0]),
                one,
                two,
                three,
                four,
                five,
                six
            ]
            .spacing(16),
        )
        .width(560)
        .height(Length::Fill)
        .padding([24, 8]);
        main_content
    }

    fn left_content(&self, ui: UI) -> Column<'_, Message> {
        let top = column![
            ui.section_heading("Connections"),
            ui.text_input("Search…", &self.search_input)
                .on_input(Message::SearchInputChanged)
        ]
        .spacing(12)
        .padding([8, 8, 0, 8]);

        let middle = column![
            ui.select_item("My first DB", mini_square(color!(0x1B9E77)), 1, false),
            ui.select_item("My Acme Database", mini_square(color!(0xE7298A)), 1, true),
            ui.select_item("Some Random One", mini_square(color!(0x7570B3)), 1, false),
            ui.select_item("Group", Space::with_width(16).into(), 1, false), // todo down icon here
            ui.select_item("Some Random One", mini_square(color!(0x7570B3)), 2, false)
        ]
        .height(Length::Fill);

        let bottom = row![
            ui.button_small(container("New connection").center_x()) // todo needs + icon
                .on_press(Message::ButtonPressed)
                .width(Length::Fill),
            ui.button_small("Group").on_press(Message::ButtonPressed) // todo needs + icon
        ]
        .spacing(8)
        .padding(8);

        column![top, middle, vertical_space(), bottom].spacing(8)
    }
}

fn colored_box<'a>(color: Color, width: u16, height: u16) -> Element<'a, Message> {
    struct Square {
        color: Color,
    }

    impl canvas::Program<Message> for Square {
        type State = ();

        fn draw(
            &self,
            _state: &Self::State,
            renderer: &Renderer,
            _theme: &Theme,
            bounds: Rectangle,
            _cursor: mouse::Cursor,
        ) -> Vec<canvas::Geometry> {
            let mut frame = canvas::Frame::new(renderer, bounds.size());
            frame.fill_rectangle(Point::ORIGIN, bounds.size(), self.color);

            vec![frame.into_geometry()]
        }
    }

    canvas(Square { color }).width(width).height(height).into()
}

fn fake_color_picker<'a>(color: Color) -> Element<'a, Message> {
    colored_box(color, 48, 24)
}

fn mini_square<'a>(color: Color) -> Element<'a, Message> {
    colored_box(color, 16, 16)
}
