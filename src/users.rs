// TODO
use crate::databases::{Connection, MainDatabase, User, DatabaseUtils};
use bson::doc;
use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use rocket_dyn_templates::{Template, context};
use rocket_csrf::CsrfToken;
use rocket::{form::Form, Route, http::{Status, Cookie, CookieJar}, response::Redirect, request::{Outcome, FromRequest}};
// TODO: use emails as the username in the future

#[derive(FromForm)]
struct LoginData<'r> {
    authenticity_token: String,
    next_page: Option<&'r str>,
    username: &'r str,
    password: &'r str,
}

struct _UserGuard {
    username: String,
    admin: bool
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for _UserGuard {
    type Error = bool;

    async fn from_request(request: &'r rocket::Request<'_>) ->  Outcome<Self, Self::Error> {
        let username = request.cookies().get_private("username").map(|c| c.value().to_string());
        let admin = request.cookies().get_private("admin").map(
            |v| match v.value() {
            "true" => true,
            _ => false
        });
        if let (Some(username), Some(admin)) = (username, admin) {
            return Outcome::Success(_UserGuard{username, admin});
        }
        Outcome::Failure((Status::Unauthorized, false))
    }
}

#[post("/login", data = "<form>")]
async fn login_post(form: Form<LoginData<'_>>, csrf_token: CsrfToken, db: Connection<MainDatabase>, jar: &CookieJar<'_>, ) -> Result<Redirect, Status> {
    if csrf_token.verify(&form.authenticity_token).is_err() {
        return Ok(Redirect::to(uri!(login(form.next_page))));
    }
    // verify login here
    if let Some(existing_user) = db.users_coll().find_one(doc!{"username": &form.username}, None).await.map_err(|_e| Status::InternalServerError)? {
        let parsed_hash = PasswordHash::new(existing_user.password_hash()).map_err(|_e| Status::InternalServerError)?;
        if Argon2::default().verify_password(&form.password.as_bytes(), &parsed_hash).is_ok() {
            // user is authenticated, add username and admin status to private cookie
            // https://rocket.rs/v0.5-rc/guide/requests/#private-cookies
            jar.add_private(Cookie::new("username", existing_user.username().to_string()));
            jar.add_private(Cookie::new("admin", existing_user.admin().to_string()));
            if let Some(next_page) = form.next_page {
                return Ok(Redirect::to(next_page.to_string()));
            }
            return Ok(Redirect::to(uri!(authenticated_sample_route)));
        }
     }
     // username or password incorrect
    // Flash::error(Redirect::to(uri!(login(form.next_page)), "invalid username or password"))
    Ok(Redirect::to(uri!(login(form.next_page))))
}

#[get("/post-login")]
fn authenticated_sample_route() -> String {
    "worked!".to_string()
}

#[get("/login?<next>", rank=1)]
fn login(csrf_token: CsrfToken, next: Option<&str>) -> Template {
    Template::render("login", context! {
        authenticity_token: csrf_token.authenticity_token(),
        next_page: next
    })
}

#[get("/login?<next>", rank=2)]
fn login_new(next: Option<&str>) -> Redirect {
    Redirect::to(uri!(login(next)))
}


#[derive(FromForm)]
struct SignUpData<'b> {
    authenticity_token: String,
    next_page: Option<&'b str>,
    username: &'b str,
    password: &'b str,
    _email_code: Option<&'b str>, // TODO
}


#[get("/sign-up?<next>", rank = 1)]
fn sign_up(csrf_token: CsrfToken, next: Option<&str>) -> Template {
    // TODO: add another method that redirects if already logged in?
    Template::render("sign-up", context! {
        authenticity_token: csrf_token.authenticity_token(),
        next_page: next
    })
}

#[get("/sign-up?<next>", rank = 2)]
fn sign_up_new(next: Option<&str>) -> Redirect {
    Redirect::to(uri!(sign_up(next)))
}

#[post("/sign-up", data = "<form>")]
async fn post_sign_up(db: Connection<MainDatabase>, csrf_token: CsrfToken, form: Form<SignUpData<'_>>) -> Result<Redirect, Template> {
    if csrf_token.verify(&form.authenticity_token).is_err() {
        return Ok(Redirect::to(uri!(sign_up(form.next_page))));
    }
    // create user by first hashing the salted password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(form.password.as_bytes(), &salt).unwrap().to_string();
    if db.users_coll().insert_one(
        User::new(
            form.username,
            &password_hash,
        ), None).await.is_err() {
        return Err(Template::render("sign-up", context! {
            authenticity_token: csrf_token.authenticity_token(),
            next_page: form.next_page,
            error: "username already exists"
        }));
    };
    // TODO: email confirmation (requires rendering template with different variables)
    if let Some(next_page) = form.next_page {
        return Ok(Redirect::to(next_page.to_string()));
    }
    Ok(Redirect::to(uri!(login(form.next_page))))
}

pub fn routes() -> Vec<Route> {
    routes![login, login_new, login_post, authenticated_sample_route, sign_up, sign_up_new, post_sign_up]
}
