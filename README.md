# About actix-identity-sample

A sample program that performs user authentication with ID and password on the Web using Actix Identity.

# Tested Environment

* OS
    * Ubuntu 20.04.6 LTS
* Rust(rustc, cargo) version
    * 1.79.0

# Start Web server

By running the following command, a web server that listens on port 8080 will start.

```
cargo run --release
```

# About each path on the Web site

| path | authentication required | role |
| --- | --- | --- |
| ``/`` | not required | Public page accessible to all |
| ``/login`` | not required | Login page. After successful authentication, redirect to ``/welcome`` |
| ``/logout`` | required | Logout page. Redirect to ``/`` |
| ``/welcome`` | required | Display user ID after authentication |
| ``/secret`` | required | Only authenticated users can access |

[AuthMiddleware](src/middleware.rs) is used to determine if a request has been authenticated by a user.

# Author
 
showchan33

# License
"actix-identity-sample" is under [GPL license](https://www.gnu.org/licenses/licenses.en.html).
