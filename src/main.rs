use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use run_app::{hrrs, trimp, trimp_lt};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
struct Run {
    avg_hr: f64,
    max_hr: f64,
    rest_hr: f64,
    workout_duration: f64,
    lt_hr: f64,
}

#[derive(Serialize)]
struct CalculationResponse {
    value: f64,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": "Hello, I am your load running calculator!"
    }))
}

#[post("/trimp")]
async fn trimp_post(run: web::Json<Run>) -> impl Responder {
    match trimp(run.avg_hr, run.max_hr, run.rest_hr, run.workout_duration) {
        Ok(value) => HttpResponse::Ok().json(CalculationResponse {
            value: (value * 100.0).round() / 100.0,
        }),
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

#[post("/trimp_lt")]
async fn trimp_lt_post(run: web::Json<Run>) -> impl Responder {
    match trimp_lt(run.lt_hr, run.max_hr, run.rest_hr, None) {
        Ok(value) => HttpResponse::Ok().json(CalculationResponse {
            value: (value * 100.0).round() / 100.0,
        }),
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

#[post("/hrrs")]
async fn hrrs_post(run: web::Json<Run>) -> impl Responder {
    match hrrs(
        run.avg_hr,
        run.max_hr,
        run.rest_hr,
        run.workout_duration,
        run.lt_hr,
    ) {
        Ok(value) => HttpResponse::Ok().json(CalculationResponse {
            value: (value * 100.0).round() / 100.0,
        }),
        Err(e) => HttpResponse::BadRequest().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://0.0.0.0:8080");

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(trimp_post)
            .service(trimp_lt_post)
            .service(hrrs_post)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
