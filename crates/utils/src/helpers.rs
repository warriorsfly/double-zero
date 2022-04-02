use actix_web::web::Json;
use serde::Serialize;

use crate::Error;

/// 快速组装Ok/Json response
pub fn respond_json<T>(data: T) -> Result<Json<T>, Error>
where
    T: Serialize,
{
    Ok(Json(data))
}
// ///
// pub fn respond_ok() -> Result<HttpResponse<Body>, Error> {
//     Ok(HttpResponse::Ok().)
// }

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
    pub struct TestResponse {
        pub first_name: String,
    }
    #[test]
    fn it_responds_json() {
        let response = TestResponse {
            first_name: "Zhang".into(),
        };

        let result = respond_json(response.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().into_inner(), response);
    }

}
