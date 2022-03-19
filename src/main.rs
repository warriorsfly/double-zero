use actix_cors::Cors;
use actix_web::{App, HttpServer};
use double_zero_utils::config::CONFIG;
use system::config_double_zero_system;
use routes::config_routes;

mod constants;
mod messages;
mod routes;
mod system;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {

        App::new()
            // 添加跨域
            .wrap(Cors::permissive())
            // .wrap(casbin_middleware.clone())
            // .wrap(InocAuth)
            // 连接数据库
            .configure(config_double_zero_system)
            // 注册路由
            .configure(config_routes)
    })
    .bind(&CONFIG.server)?
    .run()
    .await
}
