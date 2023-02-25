// TODO
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use rocket_csrf::CsrfToken;
use rocket::{form::Form, Route, http::{uri::Origin, Status}, response::Redirect, futures::TryStreamExt};
use rocket_db_pools::mongodb::options::FindOptions;
use crate::databases::{Connection, MainDatabase, DatabaseUtils};
use rocket_dyn_templates::{Template, context};
use bson::{self, doc, oid::ObjectId};

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
async fn blog_posts(db: Connection<MainDatabase>, page: Option<u64>) -> Template {
    let posts_coll = db.app_db().collection::<BlogPost>("posts");
    let post_to_skip = (page.unwrap_or(1).clamp(1, u64::MAX / 50 + 1) - 1) * 50;
    let find_options = FindOptions::builder().limit(50).skip(post_to_skip).sort(doc! {"published_time": -1}).build();
    let cursor = posts_coll.find(None, find_options).await.unwrap();
    let posts: Vec<BlogPost> = cursor.try_collect().await.unwrap();
    // can probably do html generation on Rust side to avoid another iteration
    Template::render("blog/index", context! {posts})
}

#[get("/posts/<id>", rank=2)]
async fn blog_post(id: &str, db: Connection<MainDatabase>) -> Result<Template, Status> {
    let posts_coll = db.app_db().collection::<BlogPost>("posts");
    let oid = ObjectId::parse_str(&id).map_err(|_e| Status::NotFound)?;
    match posts_coll.find_one(doc!{"_id": oid}, None).await.unwrap() {
        Some(post) => Ok(Template::render("blog/post", context!{post})),
        _ => Err(Status::NotFound)
    }
}

#[get("/new-post", rank=1)]
fn new_blog_post(csrf_token: CsrfToken) -> Template {
    Template::render("blog/new_post", context! {
        authenticity_token: csrf_token.authenticity_token(),
    })
}

#[get("/new-post", rank=2)]
fn new_blog_post_redirect() -> Redirect {
    Redirect::to(uri!(BASE, new_blog_post()))
}

#[post("/new-post", data = "<form>")]
async fn new_blog_post_api(form: Form<PostData<'_>>, csrf_token: CsrfToken, db: Connection<MainDatabase>) -> Result<Redirect, Status> {
    if csrf_token.verify(&form.authenticity_token).is_err() {
        return Err(Status::Unauthorized);
    }
    let posts = db.database("app_name").collection("posts");
    let published_time: DateTime<Utc> = Utc::now().into();
    let published_str = format!("{}", published_time.format("%Y-%b-%d"));
    let _insert_result = posts.insert_one(
        bson::to_document(
            &BlogPost{_id: ObjectId::new(), title: form.title.into(), content: form.content.into(), published_time, published_str}
        ).unwrap(), None).await.map_err(|_e| Status::InternalServerError)?;
    // TODO: flash a success
    Ok(Redirect::to(uri!(BASE, blog_posts(Some(1)))))
}

pub fn routes() -> Vec<Route> {
    routes![blog_posts, blog_post, new_blog_post, new_blog_post_redirect, new_blog_post_api]
}
