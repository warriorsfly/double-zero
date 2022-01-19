use serde::Serialize;

mod assignee;
mod comment;
mod event;
mod follower;
mod remind;
mod task;
mod todo;

#[derive(Serialize)]
pub struct Resource<T>
where
    T:Serialize
{
    code:i32,
    msg:String,
    data:Option<T>
}
