// admin.rs

// includes admin related code
use rocket::http::Status;
use rocket::request::{self, Outcome, Request, FromRequest};

#[derive(Debug)]
struct AdminUser<'r>(&'r str);
// https://api.rocket.rs/v0.5-rc/rocket/request/trait.FromRequest.html
// https://api.rocket.rs/v0.5-rc/rocket/request/trait.FromRequest.html#example-1


// theme.rs

#[derive(Debug)]
enum AdminUser{
    Light,
    Dark
}
