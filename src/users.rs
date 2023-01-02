use rocket_dyn_templates::{Template, context};
use rocket::form::Form;
use rocket::Route;
use rocket_csrf::CsrfToken;
use rocket::response::Redirect;

use crate::databases::{Connection, MainDatabase, doc, User, DatabaseUtils};

// TODO: use emails as the username in the future

#[derive(FromForm)]
struct LoginData<'r> {
    authenticity_token: String,
    next_page: Option<&'r str>,
    username: &'r str,
    password: &'r str,
}

#[post("/login", data = "<form>")]
fn login_post(db: Connection<MainDatabase>, csrf_token: CsrfToken, form: Form<LoginData>) -> Redirect {
    if csrf_token.verify(&form.authenticity_token).is_err() {
        return Redirect::to(uri!(login(form.next_page)));
    }
    // verify login in next video
    // Ok(())
    if let Some(next_page) = form.next_page {
        return Redirect::to(next_page.to_string());
    }
    Redirect::to(uri!(default_post_login))
}

#[get("/post-login")]
fn default_post_login() -> String {
    "worked!".to_string()
}

#[get("/login?<next>")]
fn login(csrf_token: CsrfToken, next: Option<&str>) -> Template {
    Template::render("login", context! {
        authenticity_token: csrf_token.authenticity_token(),
        next_page: next
    })
}

#[get("/login?<next>", rank=1)]
fn login_new(next: Option<&str>) -> Redirect {
    Redirect::to(uri!(login(next)))
}


#[derive(FromForm)]
struct SignUpData<'b> {
    authenticity_token: String,
    next_page: Option<&'b str>,
    username: &'b str,
    password: &'b str,
    email_code: Option<&'b str>,
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
    // try creating user
    // TODO: hash password
    // TODO: set admin false by default after first user
    if db.users_coll().insert_one(
        User::new(
            form.username,
            form.password,
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
    Ok(Redirect::to(uri!(default_post_login)))
}

pub fn routes() -> Vec<Route> {
    routes![login, login_new, login_post, default_post_login, sign_up, sign_up_new, post_sign_up]
}
