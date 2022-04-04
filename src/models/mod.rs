use crate::schema::games;
use crate::schema::users;

#[derive(Identifiable, Queryable)]
pub struct Game {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub user_count: i32,
}

#[derive(Insertable)]
#[table_name="games"]
pub struct NewGame {
    pub code: String,
    pub name: String,
    pub user_count: i32,
}

impl NewGame {
    pub fn new(code: &str, name: &str, user_count: i32) -> NewGame {
        NewGame {
            code: code.to_string(),
            name: name.to_string(),
            user_count
        }
    }
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Game)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub game_id: i32,
    pub monday: bool,
    pub tuesday: bool,
    pub wednesday: bool,
    pub thursday: bool,
    pub friday: bool,
    pub saturday: bool,
    pub sunday: bool,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub name: String,
    pub game_id: i32,
    pub monday: bool,
    pub tuesday: bool,
    pub wednesday: bool,
    pub thursday: bool,
    pub friday: bool,
    pub saturday: bool,
    pub sunday: bool,
}

impl NewUser {
    pub fn new(name: &str, game_id: i32, weekdays: Vec<String>) -> NewUser {
        let mut user = NewUser {
            name: name.to_string(),
            game_id,
            monday: false,
            tuesday: false,
            wednesday: false,
            thursday: false,
            friday: false,
            saturday: false,
            sunday: false,
        };

        for day in weekdays {
            match &day.to_lowercase()[..] {
                "mon" => user.monday = true,
                "tue" => user.tuesday = true,
                "wed" => user.wednesday = true,
                "thu" => user.thursday = true,
                "fri" => user.friday = true,
                "sat" => user.saturday = true,
                "sun" => user.sunday = true,
                _ => {}
            }
        }

        user
    }
}
