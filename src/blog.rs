use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveDateTime};
use rocket_csrf::CsrfToken;
use rocket::{form::Form, Route, http::{uri::Origin, Status}, response::Redirect, futures::{TryStreamExt, StreamExt}, serde::json::to_pretty_string};
use rocket_db_pools::mongodb::{options::FindOptions, TopologyType::Unknown};
use crate::databases::{Id, Connection, MainDatabase, DatabaseUtils};
use rocket_dyn_templates::{Template, context};
use bson::{self, doc, Document, oid::ObjectId, serde_helpers::bson_datetime_as_rfc3339_string};

pub const BASE: Origin<'static> = uri!("/blog");

#[derive(FromForm)]
struct PostData<'r> {
    authenticity_token: String,
    title: &'r str,
    content: &'r str,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct BlogPost {
    _id: ObjectId,
    title: String,
    content: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    published_time: DateTime<Utc>,
    published_str: String
}

#[get("/?<page>")]
fn index(page: Option<u64>) -> Redirect {
    Redirect::to(uri!(BASE, blog_posts(page)))
}

#[get("/posts?<page>")]
async fn blog_posts(db: Connection<MainDatabase>, page: Option<u64>) -> Template {
    let posts_coll = db.app_db().collection::<BlogPost>("posts");
    let find_options = FindOptions::builder().limit(50).skip((page.unwrap_or(1) - 1) * 50).build();
    let cursor = posts_coll.find(None, find_options).await.unwrap();
    let posts: Vec<BlogPost> = cursor.try_collect().await.unwrap();
    // can probably do html generation on Rust side to avoid another iteration
    Template::render("blog/index", context! {posts})
}

#[get("/posts/<id>")]
async fn blog_post(id: &str, db: Connection<MainDatabase>) -> Result<Template, Status> {
    let posts_coll = db.app_db().collection::<BlogPost>("posts");
    let oid = ObjectId::parse_str(&id).map_err(|_e| Status::InternalServerError)?;
    match posts_coll.find_one(doc!{"_id": oid}, None).await.unwrap() {
        Some(post) => Ok(Template::render("blog/post", context!{post})),
        _ => Err(Status::NotFound)
    }
}

#[get("/posts/new")]
fn new_blog_post(csrf_token: CsrfToken) -> Template {
    Template::render("blog/new", context! {
        authenticity_token: csrf_token.authenticity_token(),
    })
}

#[get("/posts/new", rank=1)]
fn new_blog_post_redirect() -> Redirect {
    Redirect::to(uri!(BASE, new_blog_post()))
}

#[post("/posts/new", data = "<form>")]
async fn new_blog_post_api(form: Form<PostData<'_>>, csrf_token: CsrfToken, db: Connection<MainDatabase>) -> Result<Redirect, Status> {
    if csrf_token.verify(&form.authenticity_token).is_err() {
        return Err(Status::Unauthorized);
    }
    let posts = db.database("app_name").collection("posts");
    let published_time: DateTime<Utc> = Utc::now().into();
    let published_str = format!("{}", published_time.format("%Y-%b-%d %H:%M"));
    let _insert_result = posts.insert_one(
        bson::to_document(
            &BlogPost{_id: ObjectId::new(), title: form.title.into(), content: form.content.into(), published_time, published_str}
        ).unwrap(), None).await.map_err(|_e| Status::InternalServerError)?;
    // TODO: flash a success
    Ok(Redirect::to(uri!(BASE, blog_posts(Some(1)))))
}

pub fn routes() -> Vec<Route> {
    routes![index, blog_posts, blog_post, new_blog_post, new_blog_post_redirect, new_blog_post_api]
}
