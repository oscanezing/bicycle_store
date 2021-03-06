use diesel::r2d2::PoolError;
use serde::Serialize;
use std::convert::Infallible;
use thiserror::Error;
use warp::{hyper::StatusCode, Rejection, Reply};

#[derive(Error, Debug)]
pub enum Error {
    #[error("error getting connection from DB pool: {0}")]
    DBPoolError(PoolError),
    #[error("error executing DB query: {0}")]
    DBQueryError(#[from] diesel::result::Error),
    #[error("invalid description")]
    BadDescriptionLen,
    #[error("invalid wheel size")]
    BadWheelSize,
}

impl warp::reject::Reject for Error {}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not found";
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = "Bad Request"
    } else if let Some(e) = err.find::<Error>() {
        eprint!("{}", e);
        match e {
            Error::DBQueryError(_) => {
                code = StatusCode::BAD_REQUEST;
                message = "Could not execute request";
            }
            Error::BadDescriptionLen => {
                code = StatusCode::BAD_REQUEST;
                message = "description must be shorter than 100 chars";
            }
            Error::BadWheelSize => {
                code = StatusCode::BAD_REQUEST;
                message = "Wheel size must be less than 30";
            }
            _ => {
                eprintln!("unhandled application error: {:?}", err);
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Internal Server Error";
            }
        }
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method Not Allowed";
    } else {
        eprint!("unhandled error: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Server error";
    }

    let json = warp::reply::json(&ErrorResponse {
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
