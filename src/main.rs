use actix_files::Files;
use actix_service::Service;
use actix_session::{Session, UserSession, CookieSession};
use actix_web::{
    cookie::SameSite,
    http::StatusCode,
    web::{get, resource},
    App, Error, HttpResponse, HttpServer,
};

async fn index(session: Session) -> Result<HttpResponse, Error> {
    let counter: i32 = session
        .get::<i32>("counter")
        .unwrap_or(Some(0))
        .unwrap_or(0);

    println!("Count: {}", counter);
    session.set("counter", counter + 1).unwrap();

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html")))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let session = CookieSession::signed(&[0; 32])
            .same_site(SameSite::Strict)
            .max_age(10800)
            .secure(false);

        App::new()
            .wrap_fn(|req, srv| {
                let _session = req.get_session();
                let fut = srv.call(req);
                async { Ok(fut.await?) }
            })
            .wrap(session)
            .service(Files::new("/static/dio.webp", "./static/dio.webp"))
            .service(resource("/").route(get().to(index)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
