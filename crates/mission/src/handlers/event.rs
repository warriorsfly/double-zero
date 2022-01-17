use serde::Deserialize;

#[derive(Deserialize)]
pub struct MissionEventForm{
    /// Task ID
    mission_id:String,
    /// Notification type
    /// - 1: mission details change
    /// - 2: mission collaborators change
    /// - 3: mission followers change
    /// - 4: mission reminder time changes
    /// - 5: mission completion
    /// - 6: mission cancellation completion
    /// - 7: mission deletion
    obj_type:i32,
}