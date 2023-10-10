# Rust Backend Tutorials Using Rocket

My goal is to be able to covert some of my website's flask backend into a rocket.rs backend without changing my templates by too much.
Doing so will make me a more proficient Rust programmer and I'll also be able to churn out a couple videos doign so.

A helpful command to debug faster

```sh
cargo 2>cargo.log
```

```sh
cargo install cargo-watch
cargo watch -x run
```

## Troubleshooting

```log
process didn't exit successfully: C:\Users\maste\Documents\GitHub\rust-backend-tutorials\target\debug\build\ring-a1678af766c289b9\build-script-build (exit code: 101)
```

To fix this, make sure you have Visual Studio installed, and run this command. You may also need to install the MSVC version of rust.

```cmd
rustup override set stable-msvc
```

## Tutorials Outline

[Rust Backend Development Part 1 - Creating an API Endpoint in Rocket](https://youtu.be/2vxvSMkm5Lg)

- Starting a Rocket project
- Variables including optional query variables
- Redirection
- URI Prefix for mounting
- Returning JSON
- Returning Two Responses

[Rust Backend Development Part 2  - Making API Requests Inside Rocket Server](https://youtu.be/Alyr-JN2pdQ)

- Rocket.rs State introduction
- Client builder (user agent)
- Client blocking issues
- Result error handling
- Making request and getting Json or text within async context

[Rust Backend Development Part 3 - JSON Manipulation](https://youtu.be/FHlCVMhNdeU)

- How to create a Json response from scratch
- Talk about [de] serialization
- how to create Arbitrary Json
- how to use values in Json
- how to add values to Json
- how to error handle with requests

[Rust Backend Development Part 4  - Caching Function Results in Rocket Server](https://youtu.be/NYYE6FgkXGI)

- Lecture on concurrency in Rust
- Using RwLock to cache endpoint results
- Arc is for multi threaded memory safety
- Mutex is for read and write exclusive access
- RwLock is for write exclusive access if multiple reads is allowed
- If you don't want to create a new type for each response, you can use an LRU Cache with a time to live (TTL) for functions but this is not as performative
- For me: look into [ignite fairing](https://api.rocket.rs/v0.5-rc/rocket/struct.Rocket.html#method.ignite) to update cached respones
in the background.

[Rust Backend Development Part 5 - Serding Structs and Organizing the Codebase](https://youtu.be/F6r3GleRewU)

- Json serialize, deserialize, and manipulate Structs
- Splitting up features into different files
- Exporting routes and base paths from files
- Importing `utils.rs` in from files other than `main.rs`
- Specifying a port using `Rocket.toml`

    ```toml
    [debug]
    port = 2000
    ```

[Rust Backend Development Part 6 - Templates, Forms, Static Files](https://youtu.be/dkh94E17bdU)

- [Templates](https://rocket.rs/v0.5-rc/guide/responses/#templates)
  - Use Tera since syntax is jinja2 meaning portability with Python
  - Creating a login page that extends a base template
  - Optimize your templates in the future by using the askama engine
- Processing form data with [CSRF Protection](https://github.com/kotovalexarian/rocket_csrf)
  - Provide fail safe routes by using rank

- [Serving static files](https://api.rocket.rs/v0.5-rc/rocket/fs/struct.FileServer.html) with path traversal protection

Rust Backend Development Part 7 - Integrating MongoDB

- Setting up [MongoDB](https://www.mongodb.com/docs/drivers/rust/) with Rocket
- Creating a REST API for blogging
  - new
  - edit
  - delete
  - posts
  - posts/id

Rust Backend Development Part 8 - Authentication & Rate Limiting

- Hashing password with bcrypt
- Creating a user API
  - Verifying if a user logged in correctly
  - bcrypt
- Verifying an admin user
- Authenticated routes
- Redirecting non-admins to the login page
- Creating a new user with MongoDB
- [Rate limiting](https://crates.io/crates/rocket-governor) for security

Rust Backend Development Part 10 - Hybrid Frontend with Templates and Webapp

- Rendering the blog page
- Conditional rendering an edit button

Rust Backend Development Part 11 - Background Functions

Rust Backend Development Part 12 - Calling Python Script or Code from Within

- Switch over to Rust immediately while relying on Python for key features
- Website or app will maintain feature set and Rust is used with a shorter waiting period
- Rely on Rust cache and commands instead of Python's lru cache
- Jinja2 parsing conversion

Rust Backend Developer Part 13 - Production Deployment (DevOps)

- Create a secret key using `openssl rand -base64 32` or with Python:

    ```py
    >>> import secrets
    >>> secrets.token_urlsafe(32)
    ```
