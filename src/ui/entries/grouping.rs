use crate::objects::author::Author;
use crate::ui::resources;
use gdk_pixbuf::Pixbuf;
use gtk::prelude::*;
use gtk::{Inhibit, Window, WindowType};
use relm::{Relm, Update, Widget};
use relm_derive::Msg;

#[derive(Msg)]
enum GroupingEntryMsg {
    GoToDetails,
    Pop,
}

pub struct GroupingEntry {
    name: &'a String,
    entries: &'a Vec<gtk::Box>,
}

#[widget]
impl Widget for GroupingEntry {
    fn model(grouping: &Grouping) -> GroupingEntry {

        GroupingEntry {
            name: grouping.name,
            enries: grouping.entries,
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