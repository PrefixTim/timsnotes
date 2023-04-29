#[macro_use] extern crate rocket;

use std::sync::Arc;

use rocket::{serde::json::Json, State};
use uuid::Uuid;
use tmn_base::{tag::Tag, content::Thought};

struct FakeDb {
    tags: Vec<Tag>,
    thoughts: Vec<Thought>
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/tag")]
fn get_tag(state: &State<Arc<FakeDb>>) -> Json<Tag> {
    Json(Tag::new(Uuid::new_v4().into(), "title".to_owned()))
}

#[launch]
fn rocket() -> _ {
    let fdb =Arc::new(FakeDb{tags: Vec::new(), thoughts: Vec::new()});
    rocket::build()
    .mount("/", routes![index, get_tag, tag_title])
    .manage(fdb)
}