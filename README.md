# Rust Backend Tutorials Using Rocket

My goal is to be able to covert some of my website's flask backend into a rocket.rs backend without changing my templates by too much.
Doing so will make me a more proficient Rust programmer and I'll also be able to churn out a couple videos doign so.

A helpful command to debug faster

```sh
cargo 2>cargo.log
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

- Using LRU Cache with a time to live (TTL)
- Arc is for multi threaded memory safety
- Mutex is for read and write exclusive access
- RwLock is for write exclusive access if multiple reads is allowed

Rust Backend Development Part 5 - Organizing Code Base

Rust Backend Development Part 6 - Rocket.rs Templates

- Bundling templates to final product
- CWD resistant

Rust Backend Development Part 8 - MongoDB

Rust Backend Development Part 9 - Post/Forms

Rust Backend Development Part 10 - CSRF Protection

Rust Backend Development Part 11 - Background Functions

Rust Backend Development Part 12 - Calling Python Script or Code from Within

- Switch over to Rust immediately while relying on Python for key features
- Website or app will maintain feature set and Rust is used with a shorter waiting period
- Rely on Rust cache and commands instead of Python's lru cache
- Jinja2 parsing conversion
