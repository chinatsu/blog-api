use super::Result;
use regex::Regex;
use hyper::{
    header,
    Body, Method, Request,
    Response, StatusCode
};

use crate::auth;
use crate::db;
use crate::db::Conn;

type DbError = diesel::result::Error;

const JSON_SERIALIZE_FAILED: &str = "Could not serialize JSON";
const DB_QUERY_FAILED: &str = "Database query failed";
const DB_ITEM_NOT_FOUND: &str = "Database item not found";
const VALIDATION_ERROR: &str = "JWT token validation error";
const MALFORMED_HEADER: &str = "Malformed header, unable to decode";
const GENERIC_OK: &str = "Cool!";

enum Route {
    Post(i32),
    Posts,
    Okay,
    Favicon,
    NotFound
}

fn get_route(input: &str) -> Route {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"/posts/(?P<id>\d+)").unwrap();
    }

    if let Some(id) = RE.captures(input).and_then(|cap| { cap.name("id").map(|id| id.as_str().parse::<i32>().unwrap()) }) {
        return Route::Post(id);
    }

    match input {
        "/favicon.ico" => Route::Favicon,
        "/posts" | "/posts/" => Route::Posts,
        "/isAlive" | "/isReady" => Route::Okay,
        _ => Route::NotFound
    }
}

pub async fn route(req: Request<Body>, conn: Conn) -> Result<Response<Body>> {
    match (req.method(), get_route(req.uri().path())) {
        (&Method::GET, Route::Favicon) => favicon(req).await,
        (&Method::GET, Route::Posts) => posts(conn).await,
        (&Method::GET, Route::Post(id)) => get_post(id, conn).await,
        (&Method::POST, Route::Posts) => add_post(req).await,
        (&Method::GET, Route::Okay) => okay(req).await,
        _ => four_oh_four().await,
    }
}

async fn okay(req: Request<Body>) -> Result<Response<Body>> {
    info!("200 OK! {}", req.uri().path());
    Ok(
        Response::builder()
            .status(StatusCode::OK)
            .body("".into())
            .unwrap()
    )
}

async fn add_post(req: Request<Body>) -> Result<Response<Body>> {
    let headers = req.headers();
    let auth_header: String = match headers.contains_key(header::AUTHORIZATION) {
        true => match headers[header::AUTHORIZATION].to_str() {
            Ok(s) => s.into(),
            Err(_) => {
                return Ok(server_error(MALFORMED_HEADER))
            }
        },
        false => return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("".into())
            .unwrap()
        )
    };
    
    let valid = match auth::validate_token(&auth_header[7..]).await {
        Ok(valid) => valid,
        Err(_) => return Ok(server_error(VALIDATION_ERROR))
    };

    if !valid {
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("".into())
            .unwrap()
        );
    }

    // TODO: Deserialize incoming object from req.body() to db::models::Post and use db::add_post
    Ok(server_error(GENERIC_OK))

}

async fn favicon(req: Request<Body>) -> Result<Response<Body>> {
    info!("200 OK! {}", req.uri().path());
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "image/png")
        .body(include_bytes!("../static/favicon.ico").to_vec().into())
        .unwrap()
    )
}

async fn four_oh_four() -> Result<Response<Body>> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body("".into())
        .unwrap()
    )
}

async fn posts(conn: Conn) -> Result<Response<Body>> {
    match db::get_all(&conn) {
        Ok(items) => match serde_json::to_string(&items) {
            Ok(json) => Ok(Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/json")
                .body(json.into())
                .unwrap()
            ),
            Err(_) => Ok(server_error(JSON_SERIALIZE_FAILED))
        }
        Err(_) => {
            Ok(server_error(DB_QUERY_FAILED))
        }
    }
}

async fn get_post(id: i32, conn: Conn) -> Result<Response<Body>> {
    match db::query(id, &conn) {
        Ok(item) => match serde_json::to_string(&item) {
            Ok(json) => Ok(Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/json")
                .body(json.into())
                .unwrap()
            ),
            Err(_) => Ok(server_error(JSON_SERIALIZE_FAILED))
        }
        Err(e) => match e {
            DbError::NotFound => Ok(server_error(DB_ITEM_NOT_FOUND)),
            _ => Ok(server_error(DB_QUERY_FAILED))
        }
    }
}

fn server_error(message: &'static str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(message.into())
        .unwrap()
}