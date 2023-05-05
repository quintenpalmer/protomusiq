use std::collections::BTreeMap;

pub fn option_to_null<T: ToString>(v: &Option<T>) -> Box<dyn rusqlite::ToSql> {
    match v {
        Some(v) => Box::new(v.to_string()),
        None => Box::new(rusqlite::types::Null),
    }
}

#[allow(unused)]
fn id_to_sql_string(id: &musiqlibrary::ID) -> Box<dyn rusqlite::ToSql> {
    Box::new(id.hashed())
}

pub fn sql_row_to_id(s: &String) -> musiqlibrary::ID {
    musiqlibrary::ID::new(s)
}

pub fn key_into_vec_by<T: Clone, F: Fn(&T) -> u32>(
    vec: Vec<T>,
    get_id: F,
) -> BTreeMap<u32, Vec<T>> {
    let mut btreemap: BTreeMap<u32, Vec<T>> = BTreeMap::new();
    for val in vec.into_iter() {
        let key = get_id(&val);
        btreemap
            .entry(key)
            .and_modify(|albums| {
                albums.push(val.clone());
            })
            .or_insert(vec![val]);
    }
    btreemap
}
