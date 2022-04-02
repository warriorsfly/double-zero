use crate::source::{LocalUser, NewLocalUser, NewUser, User};
use diesel::{prelude::*, dsl::*};
use double_zero_utils::{
    encryption::{hash_password, verify_password},
    pool::DbPool,
    Error,
};

pub fn signup<'a>(pool: &'a DbPool, username: &'a str, pwd: &'a str) -> Result<User, Error> {
    use crate::schema::local_users::dsl::*;
    use crate::schema::users::dsl::*;
    let mut conn = pool.get()?;

    let existed = select(exists(users.filter(name.eq(username)))).get_result(&mut conn);
    if let Ok(true) = existed {
        return Err(Error::BadRequest("username existed".to_string()));
    } 
    let usr = NewUser {
        name: username,
        bio: "",
        avatar: "",
    };

    let psw_hashed = hash_password(pwd).map_err(|e| Error::InternalServerError(e.to_string()))?;

    conn.transaction::<User, _, _>(|c| {
        let user = diesel::insert_into(users)
            .values(usr)
            .get_result::<User>(c)?;
        
        let local_user = NewLocalUser {
            user_id: &user.id,
            password_encrypted: &psw_hashed.0,
            salt: &psw_hashed.1,
        };
        let _ = diesel::insert_into(local_users)
            .values(local_user)
            .execute(c)?;

        Ok(user)
    })
}

pub fn login<'a>(pool: &'a DbPool, username: &'a str, psw: &'a str) -> Result<User, Error> {
    use crate::schema::local_users::dsl::*;
    use crate::schema::users::dsl::*;
    let mut conn = pool.get()?;

    conn.transaction::<User, _, _>(|c| {
        let user: User = users.filter(name.eq(username)).get_result(c)?;

        let local_user: LocalUser = local_users.filter(user_id.eq(&user.id)).get_result(c)?;

        verify_password(
            &local_user.password_encrypted,
            psw,
            local_user.salt.as_bytes(),
        )
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

        Ok(user)
    })
}
