use http::status::StatusCode;
use shared_stuff::ErrorMessage;

use warp::reject::Rejection;
use warp::reply::Reply;

pub type Result<T> = std::result::Result<T, warp::Rejection>;

#[derive(Debug, PartialEq, Eq)]
pub enum WarpRejections {
    SerializationError,
    AutocompleteError,
    UTF8Error,
    BodyDeserializeError,
    AuthRejection(AuthError),
    SqlxRejection(SqlxError),
    Other(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum AuthError {
    TokenError,
    HasherError,
    VerifyError,
}
#[derive(Debug, PartialEq, Eq)]
pub enum SqlxError {
    InsertUserError,
    CreateDBError,
    CreateTableError,
    FetchUserError,
    DBConnectionError,
    CheckLoginError,
    SelectAllUsersError,
    HasherError,
    Other,
}

impl warp::reject::Reject for WarpRejections {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply> {
    log::info!("{:?}", &err);
    let code;
    let message;
    if let Some(e) = err.find::<WarpRejections>() {
        code = StatusCode::BAD_REQUEST;
        message = format!("{:?}", e);
    } else if let Some(_e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = format!("{:?}", WarpRejections::BodyDeserializeError);
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = format!("unhandled error: {:?}", err);
    }

    let reply = warp::reply::json(&ErrorMessage {
        code: code.into(),
        message,
    });

    Ok(warp::reply::with_status(reply, code))
}
