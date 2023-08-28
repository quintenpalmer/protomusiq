use crate::model;

use super::sqlitebacked;

pub enum Loader {
    NoCache,
    JSON,
    Sqlite(sqlitebacked::Connections),
    Latest(sqlitebacked::Connections),
}

impl Loader {
    pub fn from_load_mode(
        config_state: model::app::AppConfigState,
        load_mode: model::LoadMode,
    ) -> Self {
        match load_mode {
            model::LoadMode::NoCache => Loader::NoCache,
            model::LoadMode::JSON => Loader::JSON,
            model::LoadMode::Sqlite => Loader::Sqlite(sqlitebacked::Connections::first_bootup(
                config_state.clone(),
            )),
            model::LoadMode::Latest => Loader::Latest(sqlitebacked::Connections::first_bootup(
                config_state.clone(),
            )),
        }
    }

    pub fn spawn_copy(&self) -> Self {
        match self {
            Loader::NoCache => Loader::NoCache,
            Loader::JSON => Loader::JSON,
            Loader::Sqlite(conn) => Loader::Sqlite(conn.spawn_connection()),
            Loader::Latest(conn) => Loader::Latest(conn.spawn_connection()),
        }
    }
}
