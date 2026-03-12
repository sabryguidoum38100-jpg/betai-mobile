use actix_web::{web, App, HttpServer, Responder, HttpResponse, middleware::Logger};
use actix_cors::Cors;
use serde::{Serialize, Deserialize};
use std::env;
use dotenvy::dotenv;

#[derive(Serialize, Deserialize, Debug)]
struct Prediction {
    match_id: String,
    home_win_prob: f32,
    draw_prob: f32,
    away_win_prob: f32,
    recommended_bet: String,
    confidence: f32,
}

fn poisson_prediction(match_id: &str) -> Prediction {
    let hash = match_id.len() as f32 * 0.123456;
    let home_prob = (hash % 1.0).max(0.2).min(0.8);
    let draw_prob = ((hash * 0.7) % 1.0).max(0.1).min(0.3);
    let away_prob = 1.0 - home_prob - draw_prob;

    let (recommended_bet, confidence) = if home_prob > draw_prob && home_prob > away_prob {
        ("1".to_string(), home_prob)
    } else if draw_prob > home_prob && draw_prob > away_prob {
        ("N".to_string(), draw_prob)
    } else {
        ("2".to_string(), away_prob)
    };

    Prediction {
        match_id: match_id.to_string(),
        home_win_prob: home_prob,
        draw_prob,
        away_win_prob: away_prob,
        recommended_bet,
        confidence: (confidence * 100.0).round() / 100.0,
    }
}

async fn get_prediction(match_id: web::Path<String>) -> impl Responder {
    let id = match_id.into_inner();
    let prediction = poisson_prediction(&id);
    HttpResponse::Ok().json(prediction)
}

async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse::<u16>().unwrap_or(8080);
    let bind_addr = format!("0.0.0.0:{}", port);

    println!("🚀 BetAI Backend starting on http://{}", bind_addr);

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .route("/health", web::get().to(health))
            .route("/predict/{match_id}", web::get().to(get_prediction))
    })
    .bind(&bind_addr)?
    .run()
    .await
}
