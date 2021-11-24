// use crate::song::Song;
use crate::ui::{Id, Model, Msg};

use tui_realm_stdlib::Paragraph;
use tuirealm::command::CmdResult;
use tuirealm::props::{
    Alignment, AttrValue, Attribute, BorderType, Borders, Color, PropPayload, PropValue, TextSpan,
};
use tuirealm::{Component, Event, MockComponent, NoUserEvent};

#[derive(MockComponent)]
pub struct Lyric {
    component: Paragraph,
}

impl Default for Lyric {
    fn default() -> Self {
        Self {
            component: Paragraph::default()
                                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::Green),
                )
                .foreground(Color::Cyan)
                .background(Color::Black)
                .title("Lyrics", Alignment::Left)
                .wrap(true)
                .text(
                    &[
                    TextSpan::new("No Lyrics available.").underlined().fg(Color::Green),
                    TextSpan::from("consectetur adipiscing elit. Praesent mauris est, vehicula et imperdiet sed, tincidunt sed est. Sed sed dui odio. Etiam nunc neque, sodales ut ex nec, tincidunt malesuada eros. Sed quis eros non felis sodales accumsan in ac risus"),
                    TextSpan::from("                       Duis augue diam, tempor vitae posuere et, tempus mattis ligula.")
                ])
               ,
        }
    }
}

impl Component<Msg, NoUserEvent> for Lyric {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let _drop = match ev {
            // Event::User(UserEvent::Loaded(prog)) => {
            //     // Update
            //     let label = format!("{:02}%", (prog * 100.0) as usize);
            //     self.attr(
            //         Attribute::Value,
            //         AttrValue::Payload(PropPayload::One(PropValue::F64(prog))),
            //     );
            //     self.attr(Attribute::Text, AttrValue::String(label));
            //     CmdResult::None
            // }
            // Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => return Some(Msg::GaugeAlfaBlur),
            // Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}

impl Model {
    pub fn update_lyric(&mut self) {
        let song = match self.current_song.clone() {
            Some(s) => s,
            None => return,
        };

        if song.lyric_frames_is_empty() {
            self.app
                .attr(
                    &Id::Lyric,
                    Attribute::Text,
                    AttrValue::Payload(PropPayload::Vec(vec![PropValue::TextSpan(
                        TextSpan::from("No lyrics available."),
                    )])),
                )
                .ok();
            return;
        }

        let mut line = String::new();
        if let Some(l) = song.parsed_lyric() {
            if l.unsynced_captions.is_empty() {
                return;
            }
            if let Some(l) = l.get_text(self.time_pos) {
                line = l;
            }
        }
        if self.lyric_line == line {
            return;
        }
        self.lyric_line = line.clone();
        self.app
            .attr(
                &Id::Lyric,
                Attribute::Text,
                AttrValue::Payload(PropPayload::Vec(vec![PropValue::TextSpan(TextSpan::from(
                    line,
                ))])),
            )
            .ok();
    }
}
