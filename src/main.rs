//! Default Compute@Edge template program.

use fastly::http::{HeaderValue, Method, StatusCode};
use fastly::{Error, Request, Response, Dictionary};
/// The name of a backend server associated with this service.
///
/// This should be changed to match the name of your own backend. See the the `Hosts` section of
/// the Fastly WASM service UI for more information.
const BACKEND_NAME: &str = "backend_name";

/// The name of a second backend associated with this service.
const OTHER_BACKEND_NAME: &str = "other_backend_name";

/// The entry point for your application.
///
/// This function is triggered when your service receives a client request. It could be used to
/// route based on the request properties (such as method or path), send the request to a backend,
/// make completely new requests, and/or generate synthetic responses.
///
/// If `main` returns an error, a 500 error response will be delivered to the client.
#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {
    // Make any desired changes to the client request.
    req.append_header("Host", HeaderValue::from_static("example.com"));

    // We can filter requests that have unexpected methods.
    const VALID_METHODS: [Method; 3] = [Method::HEAD, Method::GET, Method::POST];
    if !(VALID_METHODS.contains(req.get_method())) {
        return Ok(Response::from_body("This method is not allowed")
            .with_status(StatusCode::METHOD_NOT_ALLOWED));
    }

    // Pattern match on the request method and path.
    match (req.get_method(), req.get_path()) {
        // If request is a `GET` to the `/` path, send a default response.
        (&Method::GET, "/") => Ok(Response::from_body("Welcome to Fastly Compute@Edge!")
            .with_status(StatusCode::OK)),

        // If request is a `GET` to the `/backend` path, send to a named backend.
        (&Method::GET, "/backend") => {
            // Request handling logic could go here...
            // E.g., send the request to an origin backend and then cache the
            // response for one minute.
            req.set_ttl(60);
            Ok(req.send(BACKEND_NAME)?)
        }

        // If request is a `GET` to a path starting with `/other/`.
        (&Method::GET, path) if path.starts_with("/other/") => {
            // Send request to a different backend and don't cache response.
            req.set_pass(true);
            Ok(req.send(OTHER_BACKEND_NAME)?)
        }

        // Catch all other requests and return a 404.
        _ => Ok(Response::from_body("The page you requested could not be found")
            .with_status(StatusCode::NOT_FOUND)),
    }
}
