use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub enum UserOpration {
    RegisterUser,
    LoginUser,
    UpdateUser,
    CreateRoom,
    UpdateRoom,
    JoinRoom,
    LeaveRoom,
    CreateTodo,
    UpdateTodo,
    DeleteTodo,
}
