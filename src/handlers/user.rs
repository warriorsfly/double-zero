use actix_web::web::{block, Data, Json};
use double_zero_schema::{
    repository,
    source::{NewUser, User},
};
use double_zero_utils::{
    claims::{create_jwt, Claims},
    encryption::hash_password,
    helpers::respond_json,
    validate::validate,
    Error,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::system::DoubleZeroSystem;

#[derive(Debug, Deserialize, Validate)]
pub struct UserForm {
    pub name: String,
    #[validate(phone(message = "phone must be a valid email"))]
    pub phone: String,
    #[validate(length(min = 6))]
    pub password: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct PatchUserForm {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginForm {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserToken {
    pub token: String,
    pub account: User,
}

pub(crate) async fn signup(
    system: Data<DoubleZeroSystem>,
    entity: Json<UserForm>,
) -> Result<Json<User>, Error> {
    validate(&entity)?;

    let usr: User =
        block(move || repository::signup(&system.pool, &entity.name,&entity.password))
            .await??;
    respond_json(usr)
}
pub async fn login(
    system: Data<DoubleZeroSystem>,
    entity: Json<LoginForm>,
) -> Result<Json<UserToken>, Error> {
    validate(&entity)?;
    let ur =
        block(move || repository::login(&system.pool, &entity.name, &entity.password)).await??;
    let claims = Claims::new(ur.id);
    let res = UserToken {
        token: create_jwt(claims)?,
        account: ur,
    };
    respond_json(res)
}

#[cfg(test)]
mod test{
    
}