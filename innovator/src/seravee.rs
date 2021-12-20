#[cfg(test)]
mod tests {

    #[actix_web::test]
    async fn request_msg() -> std::io::Result<()> {
        use crate::activity::{self, activity_source_client::ActivitySourceClient};

        let mut client = ActivitySourceClient::connect("http://[::1]:50051")
            .await
            .expect("error connection");

        let request = tonic::Request::new(activity::Message {
            message: Some(activity::Activity {
                activity_type: "event".to_string(),
                content: "{\"subject\":\"Allen\",\"act\":\"love\",\"object\":\"rust\"}".to_string(),
            }),

            receivers: vec!["gandum".to_string(), "00".to_string()],
        });

        let response = client.active(request).await.expect("error request");
        println!("RESPONSE={:?}", response);
        Ok(())
    }
}
