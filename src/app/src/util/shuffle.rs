use rand::{self, seq::SliceRandom};

pub fn shuffle<T>(mut list: Vec<T>) -> Vec<T> {
    let mut rng = rand::thread_rng();
    list.shuffle(&mut rng);
    list
}
