// example of serializin
use rocket::response::status::NotFound;
use rocket::serde::json::{json, Value, Json};
use serde::{Serialize, Deserialize};
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Clone)]
struct Name<'r> {
    first: Cow<'r, str>,
    last: Cow<'r, str>
}

#[derive(Serialize, Deserialize, Clone)]
struct Names<'r>(Vec<Name<'r>>);

#[get("/get-api")]
async fn new_index(client: &State<Client>) -> Result<Json<Names<'_>>, NotFound<String>> {
    // make a request
    let response = client.get("url that returns a list of names").send().await.map_err(|e| NotFound(e.to_string()))?;
    let mut names = response.json::<Names<'_>>().await.map_err(|e| NotFound(e.to_string()))?;
    names.0.push(
        Name {
            first: "Rachel".into(),
            last: "Matthews".into()
        }
    );
    Ok(Json(names))
}

#[post("/post-api", data = "<names>")]
fn post_index(mut names: Json<Names<'_>>) -> Json<Names<'_>> {
    names.0.0.push(
        Name {
            first: "Rachel".into(),
            last: "Matthews".into()
        }
    );
    names
}
