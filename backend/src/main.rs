use actix_web::{App, HttpResponse, HttpServer, get};

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().body("ok")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_web::test]
    async fn test_health() {
        let app = test::init_service(App::new().service(health)).await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
