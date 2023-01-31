use std::collections::HashMap;
use rocket::http::{uri::Origin, Status};
use rocket::serde::json::{json, Value};
use rocket::{Route, State};
use reqwest;
use reqwest::Client;
use crate::utils::{StringValueCache, remove_suffix, text_request};


// TTL: time to live
pub const TTL: u64 = 5 * 60;

pub const BASE: Origin<'static> = uri!("/tauri-releases");

const GOOGLE_KEEP_DESKTOP_REPO: &str = "elibroftw/google-keep-desktop-app";

pub struct TauriGHReleaseCache{ pub mutex: StringValueCache }

pub fn routes() -> Vec<Route> {
    routes![google_keep_desktop_api]
}

async fn create_tauri_response(client: &State<Client>, github_release: &Value) -> Option<Value> {
    let platforms_available: HashMap<&str, Vec<&str>> = HashMap::from([
        ("amd64.AppImage.tar.gz", vec!["linux-x86_64"]),
        ("app.tar.gz", vec!["darwin-x86_64", "darwin-aarch64"]),
        ("x64_en-US.msi.zip", vec!["windows-x86_64"]),
    ]);

    let mut response = json!({
        "version": github_release["tag_name"].as_str()?,
        "notes": remove_suffix(&github_release["body"].as_str()?, "See the assets to download this version and install.").trim_end_matches(['\r', '\n', ' ']),
        "pub_date": github_release["published_at"].as_str()?,
        "platforms": {}
    });

    let response_platforms = response["platforms"].as_object_mut()?;
    for asset in github_release["assets"].as_array()?.iter() {
        let asset = asset.as_object()?;
        let asset_name = asset["name"].as_str()?;
        let browser_download_url = asset["browser_download_url"].as_str()?;
        for (extension, os_archs) in platforms_available.iter() {
            if asset_name.ends_with(extension) {
                for os_arch in os_archs.iter() {
                    if !response_platforms.contains_key(*os_arch) {
                        response_platforms.insert(os_arch.to_string(), json!({}));
                    }
                    response_platforms[*os_arch].as_object_mut()?.insert("url".to_string(), Value::String(browser_download_url.to_string()));
                }
            } else if asset_name.ends_with(&format!("{extension}.sig")) {
                let signature = match text_request(client, browser_download_url).await {
                    Ok(s) => s,
                    _ => String::new()
                };
                for os_arch in os_archs.iter() {
                    if !response_platforms.contains_key(*os_arch) {
                        response_platforms.insert(os_arch.to_string(), json!({}));
                    }
                    response_platforms[*os_arch].as_object_mut()?.insert("signature".to_string(), Value::String(signature.clone()));
                }
            }
        }
    }
    Some(response)
}

async fn get_latest_release(client: &State<Client>, repo: &str) -> Result<Value, reqwest::Error> {
    // repo: e.g. elibroftw/google-keep-desktop-app
    let url = format!("https://api.github.com/repos/{repo}/releases/latest");
    let response = client.get(&url).send().await?;
    let github_release = response.json::<Value>().await?;
    create_tauri_response(client, &github_release).await.ok_or(json!({})).or_else(|e| Ok(e))
}

async fn get_latest_release_ttl(cache: &State<TauriGHReleaseCache>, client: &State<Client>, repo: &str) -> Value {
    let mut guard = cache.mutex.lock().await;
    if let Some(release) = guard.get(repo) {
        return release.clone();
    }

    let release = get_latest_release(client, repo).await.or_else(|error| {
        // TODO: notify someone via Element/Discord/Slack (use webhooks or bots)
        println!("{error:?}");
        // avoid rate limiting by using empty JSON on a request error
        Ok::<Value, reqwest::Error>(json!({}))
    }).unwrap();
    guard.insert(repo.to_string(), release.clone());
    release
}

// /tauri-releases/google-keep-desktop/win64/1.18.0?msg=""
#[get("/google-keep-desktop/<_platform>/<current_version>?<msg>")] // &<other_stuff>
pub async fn google_keep_desktop_api(_platform: &str, current_version: &str, msg: Option<&str>,
                                 client: &State<Client>, cache: &State<TauriGHReleaseCache>) -> Result<Value, Status> {
    // Status::NoContent
    // error prone logic -> Option / Result
    if let Some(msg) = msg {
        println!("{msg}");
        return Err(Status::NoContent);
    }
    let latest_release = get_latest_release_ttl(cache, client, GOOGLE_KEEP_DESKTOP_REPO).await;
    // input checks. use closure to generalize Nones
    let response = move || -> Option<_> {
        let semvers: Vec<&str> = current_version.split('.').collect();
        let cur_maj = semvers.get(0)?;
        let cur_min = semvers.get(1)?;
        let cur_patch = semvers.get(2)?;
        let mut latest_version = latest_release["version"].as_str()?;
        latest_version = latest_version.trim_start_matches('v');
        let semvers: Vec<&str> = latest_version.split('.').collect();
        let latest_maj = semvers.get(0)?;
        let latest_min = semvers.get(1)?;
        let latest_patch = semvers.get(2)?;
        if cur_maj >= latest_maj && cur_min >= latest_min && cur_patch >= latest_patch {
            return None;
        }
        // NOTE: can do platform and additional version checks here
        return Some(latest_release);
    }();
    response.ok_or(Status::NoContent)
}
