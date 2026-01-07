use rand::Rng;

pub fn get_number(min: i16, max: i16) -> i16 {
    let mut rng = rand::rng();
    let random_number: i16 = rng.random_range(min..max);
    return random_number;
}