use crate::objects::game::Game;
use crate::ui::resources;
use crate::ui::widgets::stars::StarView;
use gdk_pixbuf::Pixbuf;
use gtk::prelude::*;
use gtk::{Inhibit, Window, WindowType};
use relm::{Relm, Update, Widget};
use relm_derive::Msg;

#[derive(Msg)]
enum GameEntryMsg {
    GoToDetails,
    GoToAuthor,
    ChangeStar(u8),
    ClearStar,
    OverflowMenu,
    Pop,
}

pub struct GameEntry {
    name: &'a String,
    author: &'a String,
    their_rating: &'a f64,
    our_rating: &'a Option<u8>,
    cover: &'a String,
    is_local: &'a bool,
}

#[widget]
impl Widget for GameEntry {
    fn model(game: &Game) -> GameEntry {
        let img = match game.cover_location {
            Some(filename) => filename,
            None => resources::no_cover, // TODO Make Blank Cover
        };

        GameEntry {
            name: game.name,
            author: game.author,
            their_rating: game.their_rating,
            our_rating: game.our_rating,
            cover: img,
            is_local: game.is_local,
        }
    }

    fn go_to_details(&self) {
        None
    }

    fn go_to_author(&self) {}
    
    fn change_rating(&mut self, stars: u8) {
        self.model.our_rating = Some(stars);
    }

    fn remove_rating(&mut self) {
        self.model.our_rating = None;
    }

    fn spawn_overflow(&mut self) {

    }

    fn delete() {
        None
    }

    fn update(&mut self, event: GameEntryMsg) {
        match event {
            ChangeStar(star) => self.change_rating(stars),
            ClearStar => self.remove_rating(),
            Pop => self.delete(),
        }
    }

    view! {
        gtk::Box {
            orientation: Vertical,

            #[name="cover"]
            gtk::Image {
                from_file: &self.model.cover,
            },

            #[name="label"]
            gtk::Box {
                orientation: Horizontal,

                #[name="details"]
                gtk::Box {
                    orientation: Vertical,

                    #[name="title"]
                    gtk::LinkButton {
                        label: &self.model.name,
                        clicked => GoToDetails,
                    }

                    #[name="author"]
                    gtk::LinkButton {
                        label: &self.model.author.name,
                        clicked => GoToAuthor,
                    }

                    #[name="stars"]
                }

                #[name="overflow_menu"]
                gtk::Button {
                    image: "view-more-symbolic",
                    clicked => OverflowMenu,
                }
            }
        }
    }
}

#[cfg(test)]
mod test {

}