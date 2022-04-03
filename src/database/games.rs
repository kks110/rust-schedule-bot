use diesel::prelude::*;
use crate::models::{Game, NewGame};
use std::error::Error;

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