use rocket::{
    response::Redirect,
    fs::{NamedFile, FileServer, relative},
    fairing::AdHoc
};
use reqwest;
use reqwest::Client;
use rocket_dyn_templates::Template;
use std::path::Path;
// local imports
mod serde_examples;
mod tauri_releases;
use tauri_releases::{new_tauri_gh_release, GoogleKeepDesktopRelease};
mod users;
mod utils; // used by other files
mod databases;
use databases::{MainDatabase, create_indexes};
use rocket_db_pools::Database;
mod blog;

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
    let google_keep_release: GoogleKeepDesktopRelease = new_tauri_gh_release();
    let reqwest_client = Client::builder().user_agent("reqwest").build().expect("reqwest client could not be built");

    rocket::build()
        .manage(reqwest_client)
        .manage(google_keep_release)
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
        .mount(blog::BASE, blog::routes())
}
