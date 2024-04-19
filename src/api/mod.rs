use serde::Serialize;

pub mod admins;

#[derive(Serialize)]
pub struct ApiResult<T: Serialize> {
    pub code: i32,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResult<T> {
    pub fn success(data: T) -> Self {
        ApiResult {
            code: 0,
            msg: Option::None,
            data: Some(data),
        }
    }

    pub fn error<E: ToString>(err: E) -> Self {
        ApiResult {
            code: 1,
            msg: Some(err.to_string()),
            data: None,
        }
    }
}
