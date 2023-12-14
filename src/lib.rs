mod database;

use std::sync::Mutex;
use bevy::app::App;
use bevy::prelude::Plugin;
use url::Url;
use crate::database::{create_redis, DatabaseConnection};

pub struct ServerPersistencePlugin(Mutex<Option<DatabaseConnection>>);

impl ServerPersistencePlugin {
    pub fn with_database_url(url: &str) -> Self {

        let parsed_url: Url = url.parse().expect("Not a valid URL");
        let conn = match parsed_url.scheme().to_lowercase().as_str() {
            "redis" | "rediss" => create_redis(parsed_url),
            _ => panic!("No database backend for URL scheme {}.", parsed_url.scheme()),
        };
        let conn = conn.expect(format!("Failed to create database connection to {}", url).as_str());
        Self(Mutex::new(Some(conn)))
    }
}

impl Plugin for ServerPersistencePlugin {
    fn build(&self, app: &mut App) {
        let maybe_connection = &mut *self.0.lock().unwrap();
        let conn = maybe_connection.take().expect("Cannot reuse ServerPersistencePlugin");
        app.insert_resource(conn);
    }
}
