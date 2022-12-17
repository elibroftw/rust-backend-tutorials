use std::sync::{Arc, Mutex};
use std::time::Duration;
use lru_time_cache::LruCache;
use rocket::serde::json::{Value};
use reqwest::Client;
use rocket::State;

pub type StringValueCache = Arc<Mutex<LruCache<String, Value>>>;
// RwLock vs. Mutex
// RwLock is more complicated because it allows unlimited reading and keeps write exclusive
// Arc is used for concurrent memory safety
// For example, in the future, we may want to make the API non-blocking and fetch the latest release in a background thread

pub fn cache_new(ttl: u64) -> StringValueCache {
    Arc::new(Mutex::new(LruCache::with_expiry_duration(Duration::from_secs(ttl))))
}

pub fn cache_get(mutex: &StringValueCache, key: &str) -> Option<Value> {
    mutex.lock().unwrap().get(key).map(|x|  x.clone())
}

pub fn cache_insert(mutex: &StringValueCache, key: &str, value: &Value) {
    mutex.lock().unwrap().insert(key.to_string(), value.clone());
}

pub fn remove_suffix<'a>(s: &'a str, suffix: &str) -> &'a str {
    match s.strip_suffix(suffix) {
        Some(s) => s,
        None => s,
    }
}

pub async fn text_request(client: &State<Client>, url: &str) -> Result<String, reqwest::Error> {
    client.get(url).send().await?.text().await
}
