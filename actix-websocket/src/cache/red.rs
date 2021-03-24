use actix::prelude::*;
use redis::Client;

pub struct Red {
    //redis client
    client: Client,
}

impl Actor for Red {
    type Context = Context<Self>;
}

impl Red {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}
