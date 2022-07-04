use diesel::prelude::*;
use crate::models::{User, NewUser};
use std::error::Error;

pub fn create_user(conn: &PgConnection, name: &str, game_id: i32, weekdays: Vec<String>) -> Result<User, Box<dyn Error>> {
    use crate::schema::users;

    let new_user = NewUser::new(name, game_id, weekdays);

    Ok(diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)?
    )
}

pub fn load_user_by_name_and_game_id(conn: &PgConnection, user_name: &str, gid: i32) -> Result<User, Box<dyn Error>> {
    use crate::schema::users::dsl::*;

    Ok(users.filter(game_id.eq(gid))
        .filter(name.eq(user_name))
        .first(conn)?)
}

pub fn load_users_by_game_id(conn: &PgConnection, gid: i32) -> Result<Vec<User>, Box<dyn Error>> {
    use crate::schema::users::dsl::*;

    Ok(users.filter(game_id.eq(gid))
        .load::<User>(conn)?)
}

pub fn load_user_count_by_game_id(conn: &PgConnection, gid: i32) -> Result<usize, Box<dyn Error>> {
    use crate::schema::users::dsl::*;

    Ok(users.filter(game_id.eq(gid))
        .load::<User>(conn)?.len())
}

fn delete_user(conn: &PgConnection, user_id: i32) -> Result<(), Box<dyn Error>> {
    use crate::schema::users::dsl::*;

    diesel::delete(users.filter(id.eq(user_id))).execute(conn)?;
    Ok(())
}

pub fn update_or_create(conn: &PgConnection, name: &str, game_id: i32, weekdays: Vec<String>) -> Result<User, Box<dyn Error>> {
    let potential_user = load_user_by_name_and_game_id(conn, name, game_id);

    if potential_user.is_ok() {
        delete_user(conn, potential_user.unwrap().id)?;
    }

    Ok(create_user(conn, name, game_id, weekdays)?)
}
