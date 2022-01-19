use serde::Deserialize;

#[derive(Deserialize)]
pub struct TaskEventForm{
    /// Task ID
    task_id:String,
    /// Notification type
    /// - 1: task details change
    /// - 2: task collaborators change
    /// - 3: task followers change
    /// - 4: task reminder time changes
    /// - 5: task completion
    /// - 6: task cancellation completion
    /// - 7: task deletion
    obj_type:i32,
}