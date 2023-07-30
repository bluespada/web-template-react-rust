use actix_web::{HttpRequest, HttpResponse, http::header::ContentType};

pub async fn frontend(_req: HttpRequest) -> std::io::Result<HttpResponse> {
    let html_body = include_str!("../index.html");
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html_body)
    )
}