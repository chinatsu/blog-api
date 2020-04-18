use super::Result;
use std::collections::HashMap;
use serde::{Serialize};
use hyper::{
    header,
    Body, Method, Request,
    Response, StatusCode
};

use crate::db;
use crate::db::Conn;

const JSON_SERIALIZE_FAILED: &str = "Could not serialize JSON";
const DB_QUERY_FAILED: &str = "Database query failed";

pub async fn route(req: Request<Body>, conn: Conn) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/favicon.ico") => favicon(req).await,
        (&Method::GET, "/articles") | (&Method::GET, "/articles/") => articles(conn).await,
        (&Method::GET, "/isAlive") | (&Method::GET, "/isReady") => okay(req).await,
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

async fn favicon(req: Request<Body>) -> Result<Response<Body>> {
    info!("200 OK! {}", req.uri().path());
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/png")
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

async fn articles(conn: Conn) -> Result<Response<Body>> {
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
        Err(_) => Ok(server_error(DB_QUERY_FAILED))
    }
}

fn server_error(message: &'static str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(message.into())
        .unwrap()
}