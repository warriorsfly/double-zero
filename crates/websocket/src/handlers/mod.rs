use actix_web::{
    web,
    Error, HttpRequest, HttpResponse,
};
use actix_web_actors::ws;

use crate::session::Session;

mod securitys;


pub async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    ws::start(Session::default(), &req, stream)
}