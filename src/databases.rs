use rocket_db_pools::databases::mongodb;
pub use rocket_db_pools::Connection;

// see https://api.rocket.rs/v0.5-rc/rocket_db_pools/ for all database alternatives
// see https://api.rocket.rs/v0.5-rc/rocket_db_pools/ for all database alternatives

// https://rocket.rs/v0.5-rc/guide/state/#databases

// mongodb::Client


#[database("api_db")]
struct ApiDatabase(mongodb::Client);

// usage

/**


#[get("...")]
async fn route(mut db: Connection<ApiDatabase>) {
}

*/
