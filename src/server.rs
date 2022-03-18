
use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};
use double_zero_utils::config::CONFIG;
use double_zero_websocket::config_trans_am_system;

pub(crate) async fn serv() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            // 添加跨域
            .wrap(Cors::permissive())
            // 添加日志
            .wrap(Logger::default())
            // .wrap(casbin_middleware.clone())
            // .wrap(InocAuth)
            // 连接数据库
            .configure(config_trans_am_system)
            .configure(config_database)
            // 注册路由
            .configure(routes)
    })
    .bind(&CONFIG.server)?
    .run()
    .await
}