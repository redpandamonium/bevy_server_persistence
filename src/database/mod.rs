mod key;

use async_std::channel;
use async_std::channel::{Receiver, Sender};
use bevy::prelude::Resource;
use bevy::tasks::{IoTaskPool, Task};
use redis::aio::ConnectionManager;
use url::Url;

#[derive(Resource)]
pub struct DatabaseConnection {
    channel: Sender<DatabaseCommand>,
    task: Task<anyhow::Result<()>>,
}

impl DatabaseConnection {
    pub fn new(channel: Sender<DatabaseCommand>, task: Task<anyhow::Result<()>>) -> Self {
        Self {
            channel,
            task,
        }
    }
}

pub enum DatabaseCommand {
    Shutdown,
}

pub(crate) fn create_redis(url: Url) -> anyhow::Result<DatabaseConnection> {

    let (tx, rx) = channel::unbounded();
    let task = IoTaskPool::get().spawn(async move {
        let client = redis::Client::open(url)?;
        let conn = client.get_connection_manager().await?;
        redis_async_loop(conn, rx).await?;
        Ok(())
    });

    Ok(DatabaseConnection {
        channel: tx,
        task,
    })
}

async fn redis_async_loop(conn: ConnectionManager, receiver: Receiver<DatabaseCommand>) -> anyhow::Result<()> {
    // TODO: Implement
    Ok(())
}