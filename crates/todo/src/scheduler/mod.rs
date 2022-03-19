use serde::{Deserialize, Serialize};
use validator::Validate;

pub mod executor;

/// Deadline setting for tasks
#[derive(Serialize, Deserialize, Default)]
pub struct Due {
    /// Timestamp of the deadline (in seconds)
    /// **Example value**: 1623124318
    time: i32,
    /// The time zone corresponding to the deadline, using the IANA Time Zone Database standard, such as Asia/Shanghai
    /// **Example value**: "Asia/Shanghai"
    /// **Default value**: `Asia/Shanghai`
    timezone: String,
    /// Mark whether the todo is an all-day todo (the deadline for all-day tasks is 0 o'clock of the UTC time of the day)
    /// **Example value**: false
    /// **Default value**: `false`
    is_all_day: bool,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Header {
    event_id: String,
    event_type: String,
    token: String,
    app_id: String,
    tenant_key: String,
}

#[derive(Serialize, Deserialize, Default, Validate)]
pub struct Href {
    /// The title corresponding to the link
    /// **Example value**: "反馈一个问题，需要协助排查"
    /// **Data validation rules**:
    /// - Length range: `0` ～ `512` characters
    #[validate(length(min = 0, max = 512))]
    title: String,
    /// Specific link address
    /// **Example value**: "https://support.feishu.com/internal/foo-bar"
    /// **Data validation rules**:
    /// - Length range: 0 ～ 1024 characters
    #[validate(url)]
    url: String,
}

/// Third-party platform source information associated with the todo
#[derive(Serialize, Deserialize, Validate)]
pub struct Origin {
    /// The name of the source of the todo import, which is used to display in the todo center details page. Please provide a dictionary, multi-language name mapping. Supported regional language names: it_it, th_th, ko_kr, es_es, ja_jp, zh_cn, id_id, zh_hk, pt_br, de_de, fr_fr, zh_tw, ru_ru, en_us, hi_in, vi_vn
    /// **Example value**: {"zh_cn": "IT 工作台", "en_us": "IT Workspace"}
    /// **Data validation rules**:Length range: `0` ～ `1024` characters
    #[validate(length(min = 0, max = 1024))]
    platform_i18n_name: String,

    /// Link to the source platform details page of the todo association
    href: Option<Href>,
}

#[derive(Serialize, Deserialize)]
pub struct Todo {
    id: i32,
    summary: String,
    /// Mission remarks
    discription: String,
    /// The access party can customize the subsidiary information binary format, using base64 encoding, and the resolution method is determined by the access party itself
    extra: String,
    /// Deadline setting for tasks
    due: Option<Due>,
    /// Third-party platform source information associated with the todo
    origin: Option<Origin>,
    /// This field is used to control whether the todo is editable in the Feishu todo center. The default is false. If it is true, the third party needs to consider whether it needs to access events to receive the change information of the todo in the todo center
    editable: bool,
    /// This field is used to store custom data that third parties need to pass through to the end, in Json format. In the value example, custom_complete field stores the jump link (href) or prompt message (tip) of the "Complete" button. PC, ios, android can be customized. The key of the tip field is the language type, and the value is the prompt message. The language type can be increased or decreased by itself. The language name of each region supported is it_it, th_th, ko_kr, es_es, ja_jp, zh_cn, id_id, zh_hk, pt_br, de_de, fr_fr, zh_tw, ru_ru, en_us, hi_in, vi_vn. The priority of href is higher than tip, and only jump without prompt when href and tip are not empty at the same time. Links and prompt messages can be customized, and the rest of the keys need to be passed according to the structure in the example
    custom: String,
}
