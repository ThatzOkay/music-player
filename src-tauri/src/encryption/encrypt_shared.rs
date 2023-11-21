use rand::seq::SliceRandom;


pub fn generate_random_salt(length: i32) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    
    let mut rng = rand::thread_rng();
    let mut charset: Vec<char> = CHARSET.iter().map(|&b| b as char).collect();
    
    charset.shuffle(&mut rng);
    
    let salt: String = charset.into_iter().take(length.try_into().unwrap()).collect();

    salt
}