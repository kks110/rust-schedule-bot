use rand::Rng;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            0123456789";
const GAME_ID_LENGTH: usize = 5;

pub fn generate() -> String {
    let mut rng = rand::thread_rng();
    (0..GAME_ID_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}