#[cfg(test)]
mod tests {

    #[actix_rt::test]
    async fn request_msg() -> std::io::Result<()> {
        use crate::grpc::{message_client::MessageClient, Msg, RespMsg};

        let mut client = MessageClient::connect("http://[::1]:50051")
            .await
            .expect("erro   r connection");

        let request = tonic::Request::new(Msg {
            content: "Hi, I'm allen".to_string(),
            receivers: vec!["gandum".to_string(), "00".to_string()],
        });

        let response = client.send_msg(request).await.expect("error request");
        println!("RESPONSE={:?}", response);
        Ok(())
    }
}
