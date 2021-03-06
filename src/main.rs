#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate askama;

extern crate atomic;
extern crate chrono;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
extern crate tera;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate poppler;
extern crate cairo;
extern crate tesseract;

use rocket_contrib::Template;
use tera::Context;
use std::path::PathBuf;
use rocket::response::NamedFile;
use std::path::Path;

mod fairings;
use fairings::pathfairing::PathFairing;

mod models;
use models::document::Document;
use models::tag::Tag;
use models::picture::Picture;

mod routes;
use routes::RoutesHandler;

mod handlers;

use models::correspondent::Correspondent;
use r2d2_postgres::PostgresConnectionManager;
use r2d2_postgres::TlsMode;
use r2d2::Pool;
use models::menuentry::MenuEntry;
use rocket::fairing::AdHoc;
use std::sync::Mutex;
use std::cell::RefCell;

#[get("/css/<file..>")]
fn css_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/css/").join(file)).ok()
}

#[get("/img/<file..>")]
fn img_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/img/").join(file)).ok()
}

fn main(){

    let manager =
        PostgresConnectionManager::new(
            "postgres://postgres:default@172.17.0.2/postgres",
            TlsMode::None).expect("Unable to connect to DB");

    let pool = Pool::new(manager).expect("Unable to create Pool");

    let rh = RoutesHandler{ pool,
        menu: vec![MenuEntry{
            name: "Home".to_string(),
            href: "/".to_string()
        }, MenuEntry{
            name: "Documents".to_string(),
            href: "/documents".to_string()
        }, MenuEntry{
            name: "Correspondents".to_string(),
            href: "/correspondents".to_string()
        }, MenuEntry{
            name: "Tags".to_string(),
            href: "/tags".to_string()
        }]};

    rocket::ignite()
        .manage(rh)
        .attach(PathFairing::new())
        .mount("/", routes![
        routes::correspondents::index,
        routes::correspondents::correspondent_single,
        routes::correspondents::correspondent_add,
        routes::correspondents::correspondent_add_post,
        routes::correspondents::correspondent_delete,
        routes::index::index,
        routes::documents::index,
        routes::documents::document_view_single,
        routes::documents::document_download_single,
        routes::documents::document_single,
        routes::documents::document_picture,
        routes::documents::document_ocr,
        routes::tags::index,
        routes::tags::tag_single,
        css_files,
        img_files,
        assets_files])
        .attach(Template::fairing())
        .launch();
}

#[get("/assets/<file..>")]
fn assets_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/").join(file)).ok()
}