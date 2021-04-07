use serde::{self, Deserialize, Serialize};
//{"YLJGDM":"320482109010001","KSDM":"01","YSGH":"7803","JZHZZJHM":"320100192806289851","JZHZZJLX":"01","IP":"","MAC":""
//,"METHOD":"offilneMsg","MSGLX":"1"}
#[derive(Deserialize, Serialize)]
pub struct Parameter {
    /// 机构编码
    #[serde(rename = "YLJGDM")]
    pub jgdm: String,

    /// 科室代码
    #[serde(rename = "KSDM")]
    pub ksdm: String,

    /// 医生工号
    #[serde(rename = "YSGH")]
    pub ysgh: String,

    /// 证件号码
    #[serde(rename = "JZHZZJHM")]
    pub zjhm: String,

    /// 证件类型
    #[serde(rename = "JZHZZJLX")]
    pub zjlx: String,

    /// ip
    #[serde(rename = "IP")]
    pub ip: String,

    /// mac
    #[serde(rename = "MAC")]
    pub mac: String,

    /// method
    #[serde(rename = "METHOD")]
    pub method: String,

    /// msglx
    #[serde(rename = "MSGLX")]
    pub msglx: String,
}

impl Parameter {
    pub fn new(jgdm: &str, ysgh: &str) -> Self {
        Self {
            jgdm: jgdm.to_owned(),
            ksdm: "01".to_string(),
            ysgh: ysgh.to_owned(),
            zjhm: "320100192806289851".to_owned(),
            zjlx: "01".to_owned(),
            ip: "".to_owned(),
            mac: "".to_owned(),
            method: "offilneMsg".to_owned(),
            msglx: "1".to_owned(),
        }
    }

    pub fn login(jgdm: &str, ysgh: &str) -> Self {
        Self {
            jgdm: jgdm.to_owned(),
            ksdm: "01".to_string(),
            ysgh: ysgh.to_owned(),
            zjhm: "320100192806289851".to_owned(),
            zjlx: "01".to_owned(),
            ip: "".to_owned(),
            mac: "".to_owned(),
            method: "login".to_owned(),
            msglx: "1".to_owned(),
        }
    }
}
