pub use mongodb::bson::doc;
use mongodb::{options::ClientOptions, options::IndexOptions, IndexModel};
use rocket::fairing;
use rocket::{Build, Rocket};
pub use rocket_db_pools::Connection;
use rocket_db_pools::{mongodb, Database};
use serde::{Deserialize, Serialize};

// see https://api.rocket.rs/v0.5-rc/rocket_db_pools/ for all database alternatives
// see https://api.rocket.rs/v0.5-rc/rocket_db_pools/ for all database alternatives

// https://rocket.rs/v0.5-rc/guide/state/#databases

// mongodb::Client

#[derive(Database)]
#[database("api_db")]
pub struct MainDatabase(mongodb::Client);
// usage ; ([mut] db: Connection<MainDatabase>, ...)


pub trait DatabaseUtils {
    fn app_db(&self) -> mongodb::Database;

    fn users_coll(&self) -> mongodb::Collection<User>;
}

impl DatabaseUtils for mongodb::Client {
    fn app_db(&self) -> mongodb::Database {
        // usage with Connection: &*db
        self.database("app_name")
    }

    fn users_coll(&self) -> mongodb::Collection<User> {
        // usage with Connection: &*db
        self.app_db().collection::<User>("users")
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    username: String,
    password: String,
    admin: bool,
}

impl User {
    pub fn new(username: &str, password: &str) -> User {
        // TODO: set admin to false after first user
        User { username: username.to_string(), password: password.to_string(), admin: true }
    }
}

pub async fn create_indexes(rocket: Rocket<Build>) -> fairing::Result {
    if let Some(db) = MainDatabase::fetch(&rocket) {
        db.0.users_coll()
            .create_index(
                IndexModel::builder()
                    .keys(doc! {"username": 1})
                    .options(IndexOptions::builder().unique(true).build())
                    .build(),
                None,
            )
            .await.ok();
        return Ok(rocket);
    }
    Err(rocket)
}
