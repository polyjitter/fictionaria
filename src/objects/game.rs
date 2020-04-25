use crate::objects::{
    author::Author,
    tag::Tag,
};

struct Game {
    name: String,
    author: Author,
    description: String,
    downloads: Vec<String>,
    their_rating: f64,
    our_rating: Option<u8>,
    cover_location: String,
    is_local: bool,
    tags: Vec<Tag>,
}