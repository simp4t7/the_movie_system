use http::status::StatusCode;
use shared_stuff::auth_structs::ErrorMessage;

use warp::reject::Rejection;
use warp::reply::Reply;

pub type Result<T> = std::result::Result<T, warp::Rejection>;

#[macro_export]
macro_rules! err_info {
    () => {
        format!("file name is: {} | line number is: {}", file!(), line!())
    };
}

#[derive(Debug, PartialEq, Eq)]
pub enum WarpRejections {
    SerializationError(String),
    UuidError(String),
    EnvError(String),
    AutocompleteError(String),
    AuthError(String),
    SqlxError(String),
    GroupNotExist(String),
    UserNotInGroup(String),
    UserNotExist(String),
    UserNotAuthorized(String),
    Other(String),
}

impl From<WarpRejections> for String {
    fn from(error: WarpRejections) -> Self {
        format!("{:?}", error)
    }
}

//#[derive(Debug, PartialEq, Eq)]
//pub enum AuthError {
//AccessError,
//TokenError,
//HasherError,
//VerifyError(String),
//NoAuthHeaderError,
//InvalidAuthHeaderError,
//}

//#[derive(Debug, PartialEq, Eq)]
//pub enum SqlxError {
//DeleteUserError,
//FindGroupIdError,
//GroupError,
//SelectUserError,
//SelectGroupError,
//SelectGroupsError,
//SaveMoviesError,
//CurrentMoviesError,
//UserDoesntExist,
//AddUserError,
//DeleteGroupError,
//CreateGroupError,
//InsertUserError,
//CreateDBError,
//CreateTableError,
//FetchUserError,
//DBConnectionError,
//CheckLoginError,
//SelectAllUsersError,
//HasherError,
//Other,
//}

impl warp::reject::Reject for WarpRejections {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply> {
    log::info!("{:?}", &err);
    let code;
    let message;
    if let Some(e) = err.find::<WarpRejections>() {
        code = StatusCode::BAD_REQUEST;
        message = format!("{:?}", e);
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
