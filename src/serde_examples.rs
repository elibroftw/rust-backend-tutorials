// This file contains examples of serializing
use std::borrow::Cow;  // clone-on-write
use reqwest::Client;
use rocket::http::uri::Origin;
use rocket::{Route, State};
use rocket::response::status::NotFound;
use rocket::serde::json::{json, Value, Json};
use serde::{Serialize, Deserialize};

pub const BASE: Origin<'static> = uri!("/serde");

pub fn routes() -> Vec<Route> {
    routes![get_json_example, post_json_example, json_value_example]
}

#[derive(Serialize, Deserialize, Clone)]
struct Name<'r> {
    first: Cow<'r, str>,
    last: Cow<'r, str>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Names<'r>(Vec<Name<'r>>);


#[get("/get-json-example")]
pub async fn get_json_example(client: &State<Client>) -> Result<Json<Names<'_>>, NotFound<String>> {
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

#[post("/post-json-example", data = "<names>")]
pub fn post_json_example(mut names: Json<Names<'_>>) -> Json<Names<'_>> {
    names.0.0.push(
        Name {
            first: "Rachel".into(),
            last: "Matthews".into()
        }
    );
    names
}

// arbitrary json
#[get("/value-example")]
pub async fn json_value_example() -> Value {
    // assume we are working with a 3rd-party API and they respond with JSON
    // where you only care about one field. Deserializing it would be a waste of effort
    // therefore, it's better to just handle the errors (either use ? or let Some(x) = ...)
    let mut response: Value = json!({
        "names": [
            {
                "first": "Elijah",
                "last": "L"
            }
        ],
    });
    // lets add a name
    // here I used unwrap because I already know the mapping
    // if you are working with arbitrary json, make sure to handle errors
    response["names"].as_array_mut().unwrap().push(json!({
        "first": "Rachel",
        "last": "Matthews"
    }));

    // example mutating a hashmap (the value insert has to be of type Value::... )
    response.as_object_mut().unwrap().insert("irrelevant".to_string(), Value::String("sample insert".to_string()));

    // return names (idk if its possible to do it without a clone)
    response["names"].clone()
}
