use serenity::framework::standard::{Args};

pub fn parse_day_registration(mut args: Args) -> Result<(String, Vec<String>), String>  {
    let game_code: String;
    let mut parsed_args = vec![];

    if let Ok(gid) = args.single::<String>() {
        game_code = gid
    } else {
        println!("Error with game id");
        return Err("Please enter a valid team name".to_string())
    }

    for _ in 0..(args.len() - 1) {
        if let Ok(day) = args.single::<String>() {
            parsed_args.push(day.to_lowercase())
        } else {
            println!("Error with days");
            return Err("Not enough days".to_string())
        }
    }

    Ok((game_code, parsed_args))
}

pub fn parse_string(mut args: Args) -> Result<String, String> {
    let name: String;

    if let Ok(s) = args.single::<String>() {
        name = s
    } else {
        return Err("Please enter a valid game code".to_string())
    }

    Ok(name)
}
