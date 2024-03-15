#[derive(Debug)]
pub enum Error {}

use crate::model;

pub fn entry_point() -> Result<(), Error> {
    let comparisons = vec![
        ("big red machine", "Big Red Machine"),
        ("about : river", "About: River"),
        ("three drops of dopamine", "three drops to heaven"),
        ("three drops of dopamine", "just three drops"),
    ];
    for (a, b) in comparisons.into_iter() {
        let diff = model::functions::levenshtein(a, b);
        println!("{}\t between {} and {}", diff, a, b);
    }
    Ok(())
}
