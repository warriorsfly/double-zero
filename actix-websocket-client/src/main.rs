use std::{thread, time::Duration};

#[actix::main]
async fn main() {
    let client: awc::Client = awc::Client::default();
    let mut c0: i32 = 0;
    loop {
        let response = client
        .post("http://192.168.3.16:8089/xtfw/api/pushMsg")
        .insert_header(("Content-Type", "application/json"))
        .send_body("[{  \"FSDXX\":{\"SSXTDM\":\"270121\",   \"SSXTMC\":\"危急值提醒\",   \"YWGNDM\":\"WJZTX\",\"YWGNMC\":\"危急值提醒\",\"YLJGDM\":\"123456\",\"KSDM\":\"11\",\"KSMC\":\"11\",\"RYGH\":\"11\",\"RYXM\":\"11\"  },  \"TXXX\":{\"TXBT\":\"消息标题\",\"TXNR\":\"消息内容\",\"ZJLX\":\"01\",\"ZJHM\":\"证件号码\",\"HZXM\":\"xx\",\"XXLCID\":\"消息流程ID\",\"XXZT\":\"1\",\"RWLX\": \"1\",\"XXSHDZ\": \"消息审核地址\",\"TXTZDZ\":\"提醒跳转地址\",\"TXYDHKDZ\":\"提醒消息阅读回馈地址\"  },  \"JSDXX\":[{\"JSDXLX\":\"3\",\"YLJGDM\":\"医疗机构代码\",\"KSDM\":\"1\",\"KSMC\":\"1\",\"RYGH\":\"1\",\"RYXM\":\"1\",\"HZXM\":\"1\",\"SFZH\":\"1\",\"SJHM\":\"1\",\"JSXXIP\":\"192.168.3.12\",\"JSXXMAC\":\"50:5B:C2:F0:31:65\"  }] }]")
        .await.unwrap();

        c0 = c0 + 1;
        println!("Response: {:?}", response);
        println!("Response c0: {:?}", c0);
        thread::sleep(Duration::from_secs(2));
    }
}
