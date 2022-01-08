use serde::{Deserialize, Serialize};
use validator::Validate;
/// Link to the source platform details page of the task association
#[derive(Serialize,Deserialize,Default,Validate)]
pub struct Href{
    /// The title corresponding to the link
    /// **Example value**: "反馈一个问题，需要协助排查"
    /// **Data validation rules**:
    /// - Length range: `0` ～ `512` characters
    #[validate(length(min = 0, max = 512))]
    title:String,
    /// Specific link address
    /// **Example value**: "https://support.feishu.com/internal/foo-bar"
    /// **Data validation rules**:
    /// - Length range: 0 ～ 1024 characters
    #[validate(url)]
    url:String,
}