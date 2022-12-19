use rocket_dyn_templates::{Template, context};
use rocket::form::Form;
use rocket::Route;
use rocket_csrf::CsrfToken;
use rocket::response::Redirect;

pub fn routes() -> Vec<Route> {
    routes![login, login_new, login_post, default_post_login]
}

#[derive(FromForm)]
struct LoginData<'r> {
    authenticity_token: String,
    next_page: Option<&'r str>,
    username: &'r str,
    password: &'r str,
}

#[post("/login", data = "<form>")]
fn login_post(csrf_token: CsrfToken, form: Form<LoginData>) -> Result<Redirect, Redirect> {
    if let Err(_) = csrf_token.verify(&form.authenticity_token) {
        return Err(Redirect::to(uri!(login(form.next_page))));
    }
    // verify login in next video
    // Ok(())
    if let Some(next_page) = form.next_page {
        return Ok(Redirect::to(next_page.to_string()));
    }
    Ok(Redirect::to(uri!(default_post_login)))
}

#[get("/post-login")]
fn default_post_login() -> String {
    "worked!".to_string()
}

#[get("/login?<next>", rank = 1)]
fn login(csrf_token: CsrfToken, next: Option<&str>) -> Template {
    Template::render("login", context! {
        authenticity_token: csrf_token.authenticity_token(),
        next_page: next
    })
}

#[get("/login?<next>", rank = 2)]
fn login_new(next: Option<&str>) -> Redirect {
    Redirect::to(uri!(login(next)))
}
