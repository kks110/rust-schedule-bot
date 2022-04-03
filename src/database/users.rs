use diesel::prelude::*;
use crate::models::{User, NewUser};
use std::error::Error;

pub fn create_user(conn: &PgConnection, name: &str, games_id: i32, weekdays: Vec<String>) -> Result<User, Box<dyn Error>> {
    use crate::schema::users;

    let new_user = NewUser::new(name, games_id, weekdays);

    Ok(diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)?
    )
}