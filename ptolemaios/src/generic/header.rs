use serde::{Deserialize, Serialize};
/// Link to the source platform details page of the task association
#[derive(Serialize,Deserialize,Default)]
pub struct Header{
    event_id:String,
    event_type:String,
    token:String,
    app_id:String,
    tenant_key:String,
}