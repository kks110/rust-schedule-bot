use diesel::prelude::*;
use crate::models::{Game, NewGame};
use std::error::Error;
use crate::database::users::load_users_by_game_id;
use crate::database::users::delete_user;

pub fn create_game(conn: &PgConnection, code: &str, name: &str, user_count: i32) -> Result<Game, Box<dyn Error>> {
    use crate::schema::games;

    let new_game = NewGame::new(code, name, user_count);

    Ok(diesel::insert_into(games::table)
        .values(&new_game)
        .get_result(conn)?
    )
}

pub fn load_game_by_code(conn: &PgConnection, game_code: &str) -> Result<Game, Box<dyn Error>> {
    use crate::schema::games::dsl::*;

    Ok(games.filter(code.eq(game_code))
        .first(conn)?)
}

pub fn load_games(conn: &PgConnection) -> Result<Vec<Game>, Box<dyn Error>> {
    use crate::schema::games::dsl::*;

    Ok(games.load::<Game>(conn)?)
}

pub fn delete_game(conn: &PgConnection, game_code: &str) -> Result<(), Box<dyn Error>> {
    use crate::schema::games::dsl::*;

    let game = load_game_by_code(conn, game_code)?;
    let users = load_users_by_game_id(conn, game.id)?;

    for user in users {
        delete_user(conn, user.id)?;
    }

    diesel::delete(games.filter(code.eq(game_code))).execute(conn)?;
    Ok(())
}
