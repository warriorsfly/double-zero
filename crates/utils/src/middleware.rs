use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures::future::{ok, LocalBoxFuture, Ready};

use crate::{claims::decode_jwt, constants};

pub struct JwtAuth;

impl<S> Transform<S, ServiceRequest> for JwtAuth
where
    S: actix_web::dev::Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtMiddleware { service })
    }
}

pub struct JwtMiddleware<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for JwtMiddleware<S>
where
    S: actix_web::dev::Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let mut authorized = false;
        if let Some(header) = req.headers().get(constants::AUTHORIZATION_PREFIX) {
            let header = header.to_str().unwrap();
            let header = header.trim_start_matches("Bearer ");
            let claims = decode_jwt(header);
            authorized = claims.is_ok();
        }
        if !authorized {
            return Box::pin(async move { Ok(req.into_response(HttpResponse::Unauthorized())) });
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
