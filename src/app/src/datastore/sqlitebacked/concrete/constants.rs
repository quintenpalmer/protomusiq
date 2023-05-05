pub enum Migration {
    TracksAndFriends,
    Prehistory,
    Livehistory,
}

impl Migration {
    pub fn get_name(&self) -> String {
        match self {
            Migration::TracksAndFriends => "tracks_and_friends",
            Migration::Prehistory => "prehistory",
            Migration::Livehistory => "livehistory",
        }
        .to_string()
    }
}
