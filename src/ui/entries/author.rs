use crate::objects::author::Author;
use crate::ui::resources;
use gdk_pixbuf::Pixbuf;
use gtk::prelude::*;
use gtk::{Inhibit, Window, WindowType};
use relm::{Relm, Update, Widget};
use relm_derive::Msg;

#[derive(Msg)]
enum AuthorEntryMsg {
    GoToDetails,
    Pop,
}

pub struct AuthorEntry {
    name: &'a String,
    portrait: &'a String,
}

#[widget]
impl Widget for AuthorEntry {
    fn model(game: &Author) -> AuthorEntry {
        let img = match game.portrait_location {
            Some(filename) => filename,
            None => resources::no_portrait, // TODO Make Blank Portrait
        };

        AuthorEntry {
            name: game.name,
            portrait: img,
        }
    }

    fn go_to_details(&self) {
        None
    }

    fn delete() {
        None
    }

    fn update(&mut self, event: AuthorEntryMsg) {
        match event {
            GoToDetails => self.go_to_details(),
            Pop => self.delete(),
        }
    }

    view! {
        gtk::Box {
            orientation: Vertical,

            #[name="cover"]
            gtk::Image {
                from_file: &self.model.portrait,
            },

            #[name="name"]
            gtk::LinkButton {
                label: &self.model.name,
                clicked => GoToDetails,
            }
        }
    }
}

#[cfg(test)]
mod test {

}