use serde::{Deserialize, Serialize};
use validator::Validate;

use super::Href;

/// Third-party platform source information associated with the task
#[derive(Serialize,Deserialize,Validate)]
pub struct Origin{

    /// The name of the source of the task import, which is used to display in the task center details page. Please provide a dictionary, multi-language name mapping. Supported regional language names: it_it, th_th, ko_kr, es_es, ja_jp, zh_cn, id_id, zh_hk, pt_br, de_de, fr_fr, zh_tw, ru_ru, en_us, hi_in, vi_vn
    /// **Example value**: {"zh_cn": "IT 工作台", "en_us": "IT Workspace"}
    /// **Data validation rules**:Length range: `0` ～ `1024` characters
    #[validate(length(min = 0, max = 1024))]
    platform_i18n_name:String,

    /// Link to the source platform details page of the task association
    href:Option<Href>,
}