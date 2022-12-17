use rocket::response::Redirect;
use reqwest;
use reqwest::Client;
// local imports
mod serde_examples;
mod tauri_releases;
use tauri_releases::TauriGHReleaseCache;
mod utils;
use utils::cache_new;

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

#[launch]
fn rocket() -> _ {
    let tauri_gh_cache = TauriGHReleaseCache{
        mutex: cache_new(tauri_releases::TTL)
    };
    let client = Client::builder().user_agent("reqwest").build().expect("reqwest client could not be built");
    rocket::build()
        .manage(client)
        .manage(tauri_gh_cache)
        .mount("/", routes![index])
        .mount(tauri_releases::BASE, tauri_releases::routes())
        .mount(serde_examples::BASE, serde_examples::routes())
}
