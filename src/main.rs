use rocket::response::Redirect;
use reqwest;
use reqwest::Client;
use rocket_dyn_templates::Template;
use rocket::fs::{FileServer, relative};
use std::path::Path;
use rocket::fs::NamedFile;
// local imports
mod serde_examples;
mod tauri_releases;
use tauri_releases::TauriGHReleaseCache;
mod utils;
use utils::cache_new;
mod users;
mod databases;
use databases::{MainDatabase, create_indexes};
use rocket_db_pools::Database;
use rocket::fairing::AdHoc;

#[macro_use]
extern crate rocket;

// if you are using a route from a crate, then you might find this useful for mounting those routes
// use rocket::http::uri::Origin;
// const BASE_FOR_PLUGIN: Origin<'static> = uri!("/plugin-path");

#[get("/")]
fn index() -> Redirect {
    let msg: Option<&str> = None;
    Redirect::to(uri!(tauri_releases::BASE, tauri_releases::google_keep_desktop_api("windows-x86_64", "v1.0.14", msg)))
}

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/favicon.ico")).await.ok()
}

#[launch]
fn rocket() -> _ {
    let tauri_gh_cache = TauriGHReleaseCache{
        mutex: cache_new(tauri_releases::TTL)
    };
    let reqwest_client = Client::builder().user_agent("reqwest").build().expect("reqwest client could not be built");

    rocket::build()
        .manage(reqwest_client)
        .manage(tauri_gh_cache)
        .attach(rocket_csrf::Fairing::default())
        .attach(Template::fairing())
        // attach databases
        .attach(MainDatabase::init())
        .attach(AdHoc::try_on_ignite("Create collection indexes", create_indexes))
        .mount("/static", FileServer::from(relative!("/static")))
        .mount("/", routes![index, favicon])
        .mount("/", users::routes())
        .mount(tauri_releases::BASE, tauri_releases::routes())
        .mount(serde_examples::BASE, serde_examples::routes())
}
