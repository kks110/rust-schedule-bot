table! {
    games (id) {
        id -> Int4,
        code -> Varchar,
        name -> Varchar,
        user_count -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        game_id -> Int4,
        monday -> Bool,
        tuesday -> Bool,
        wednesday -> Bool,
        thursday -> Bool,
        friday -> Bool,
        saturday -> Bool,
        sunday -> Bool,
    }
}

joinable!(users -> games (game_id));

allow_tables_to_appear_in_same_query!(
    games,
    users,
);
