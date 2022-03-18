
use actix_web::web;


pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/users")
                    .route("signup", web::post().to(user::signup))
                    .route("login", web::post().to(user::login)),
            )
            .service(
                web::scope("/rooms")
                    // .route(
                    //     "/{slug}/episodes/{page_index}/{page_size}",
                    //     web::get().to(book::get_book_episodes),
                    // )
                    
            ).service(
                web::scope("/tasks")
                    // .route(
                    //     "/{slug}/episodes/{page_index}/{page_size}",
                    //     web::get().to(book::get_book_episodes),
                    // )
                    // .route("/search/{param}", web::get().to(book::search))
                    // .route("/update/{weekday}/{page_index}/{page_size}", web::get().to(book::books_of_weekday)),
            ),
    ).service(route('/ws', ws::ws_route));
}
