use serde::{Deserialize, Serialize};

/// Deadline setting for tasks
#[derive(Serialize,Deserialize,Default)]
pub struct Due{
    /// Timestamp of the deadline (in seconds)
    /// **Example value**: 1623124318
    time:i32,
    /// The time zone corresponding to the deadline, using the IANA Time Zone Database standard, such as Asia/Shanghai
    /// **Example value**: "Asia/Shanghai"
    /// **Default value**: `Asia/Shanghai`
    timezone:String,
    /// Mark whether the task is an all-day task (the deadline for all-day tasks is 0 o'clock of the UTC time of the day)
    /// **Example value**: false
    /// **Default value**: `false`
    is_all_day:bool,
}