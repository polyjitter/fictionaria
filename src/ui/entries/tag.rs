use crate::objects::tag::Tag;
use crate::ui::resources;
use gdk_pixbuf::Pixbuf;
use gtk::prelude::*;
use gtk::{Inhibit, Window, WindowType};
use relm::{Relm, Update, Widget};
use relm_derive::Msg;

#[derive(Msg)]
enum TagEntryMsg {
    GoToDetails,
    Pop,
}

pub struct TagEntry {
    name: &'a String,
}

#[widget]
impl Widget for TagEntry {
    fn model(game: &Tag) -> TagEntry {

        TagEntry {
            name: game.name,
        }
    }

    fn go_to_details(&self) {
        None
    }

    fn delete() {
        None
    }

    fn update(&mut self, event: TagEntryMsg) {
        match event {
            GoToDetails => self.go_to_details(),
            Pop => self.delete(),
        }
    }

    view! {
        gtk::Box {
            orientation: Horizontal,

            #[name="icon"]
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