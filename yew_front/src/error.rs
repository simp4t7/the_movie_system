use thiserror::Error as ThisError;

#[derive(ThisError, Clone, PartialEq, Debug)]
pub enum Error {

    /// 401
    #[error("Unauthorized")]
    Unauthorized,

    /// 403
    #[error("Forbidden")]
    Forbidden,

    /// 404
    #[error("Not Found")]
    NotFound,

//    /// 422
//    #[error("Unprocessable Entity: {0:?}")]
//    UnprocessableEntity(ErrorInfo),

    /// 500
    #[error("Internal Server Error")]
    InternalServerError,

    /// serde deserialize error
    #[error("Deserialize Error")]
    DeserializeError,

    /// request error
    #[error("Http Request Error")]
    RequestError,

    #[error("Repeated password doesn't match")]
    PasswordNotMatchError,

    #[error("Stupid password")]
    PasswordWeakError,

    #[error("Login Error")]
    LogInError,
}

