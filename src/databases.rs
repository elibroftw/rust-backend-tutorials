use rocket::fairing;
use rocket::{Build, Rocket};
pub use rocket_db_pools::Connection;
use rocket_db_pools::{mongodb, Database};
use bson::{doc, oid::ObjectId};
use mongodb::{options::IndexOptions, IndexModel};
use serde::{Deserialize, Serialize};

pub const MAIN_DATABASE_NAME: &'static str = "app_name";

#[derive(Database)]
#[database("api_db")]
pub struct MainDatabase(mongodb::Client);
// usage : ([mut] db: Connection<MainDatabase>, ...)

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Id{
    pub _id: ObjectId
}

pub trait DatabaseUtils {
    fn app_db(&self) -> mongodb::Database;

    fn users_coll(&self) -> mongodb::Collection<User>;
}

impl DatabaseUtils for mongodb::Client {
    fn app_db(&self) -> mongodb::Database {
        self.database(MAIN_DATABASE_NAME)
    }

    fn users_coll(&self) -> mongodb::Collection<User> {
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
    pub fn new<S: Into<String>>(username: S, password: S) -> User {
        User { username: username.into(), password: password.into(), admin: false }
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password_hash(&self) -> &str {
        &self.password
    }

    pub fn admin(&self) -> bool {
        self.admin
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
