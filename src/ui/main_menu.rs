use super::data::Msg;
use tui_realm_stdlib::List;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::props::{Alignment, BorderType, Borders, Color, TableBuilder, TextSpan};
use tuirealm::{
    event::{Key, KeyEvent},
    Component, Event, MockComponent, NoUserEvent,
};

#[derive(MockComponent)]
pub struct MainMenu {
    component: List,
}

impl Default for MainMenu {
    fn default() -> Self {
        Self {
            component: List::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::Yellow),
                )
                .title("👾 wolet 👾", Alignment::Center)
                .scroll(true)
                .highlighted_color(Color::LightYellow)
                .highlighted_str("🗝️ ")
                .rewind(true)
                .step(4)
                .rows(
                    TableBuilder::default()
                        .add_col(TextSpan::from("01").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Load Existing"))
                        .add_row()
                        .add_col(TextSpan::from("02").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Create New (will overwrite old wallet)"))
                        .build(),
                )
                .selected_line(2),
        }
    }
}

impl Component<Msg, NoUserEvent> for MainMenu {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => self.perform(Cmd::Move(Direction::Down)),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                self.perform(Cmd::Move(Direction::Up))
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                ..
            }) => self.perform(Cmd::Scroll(Direction::Down)),
            Event::Keyboard(KeyEvent {
                code: Key::PageUp, ..
            }) => self.perform(Cmd::Scroll(Direction::Up)),
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => self.perform(Cmd::GoTo(Position::Begin)),
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End))
            }
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => return Some(Msg::MainMenuBlur),
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                let index = self.component.states.list_index;
                return Some(Msg::OptionSelected(index));
            }
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}
