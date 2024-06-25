use actix_web::HttpResponse;

#[tracing::instrument(name = "Confirm a pending subscriber")]
pub async fn confirm() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// #[tracing::instrument(name = "Confirm a pending subscriber", skip(_parameters))]
// pub async fn confirm(_parameters: web::Query<Parameters>) -> HttpResponse {
//     HttpResponse::Ok().finish()
// }
//

