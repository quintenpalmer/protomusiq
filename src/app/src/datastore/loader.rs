use super::sqlitebacked;

pub enum Loader {
    NoCache,
    Json,
    Sqlite(sqlitebacked::Connections),
    Latest(sqlitebacked::Connections),
}

impl Loader {
    pub fn from_load_mode(
        config_state: musiqcore::model::app::AppConfigState,
        load_mode: musiqcore::model::LoadMode,
    ) -> Self {
        match load_mode {
            musiqcore::model::LoadMode::NoCache => Loader::NoCache,
            musiqcore::model::LoadMode::Json => Loader::Json,
            musiqcore::model::LoadMode::Sqlite => Loader::Sqlite(
                sqlitebacked::Connections::first_bootup(config_state.clone()),
            ),
            musiqcore::model::LoadMode::Latest => Loader::Latest(
                sqlitebacked::Connections::first_bootup(config_state.clone()),
            ),
        }
    }

    pub fn spawn_copy(&self) -> Self {
        match self {
            Loader::NoCache => Loader::NoCache,
            Loader::Json => Loader::Json,
            Loader::Sqlite(conn) => Loader::Sqlite(conn.spawn_connection()),
            Loader::Latest(conn) => Loader::Latest(conn.spawn_connection()),
        }
    }
}
