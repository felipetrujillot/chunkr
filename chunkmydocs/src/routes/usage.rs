use crate::models::api::api_key::{ApiKey, ApiKeyLimit, ApiKeyUsage, ApiRequest, ServiceType};
use crate::models::extraction::api::ApiInfo;
use crate::utils::db::deadpool_postgres::{Client, Pool};
use actix_web::{web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct UsageResponse {
    email: String,
    key: String,
    total_usage: i32,
    usage_limit: i32,
    usage_percentage: f64,
}

/// Get Usage
///
/// Retrieve the total API usage and usage limit for the authenticated user
#[utoipa::path(
    get,
    path = "/usage",
    context_path = "/api",
    tag = "usage",
    responses(
        (status = 200, description = "Successfully retrieved usage information", body = UsageResponse),
        (status = 500, description = "Internal server error", body = String),
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn get_usage(
    pool: web::Data<Pool>,
    api_info: web::ReqData<ApiInfo>,
) -> Result<HttpResponse, Error> {
    let user_id = api_info.user_id.clone();
    let api_key = api_info.api_key.clone();

    let mut client: Client = pool.get().await.map_err(|e| {
        eprintln!("Error connecting to database: {:?}", e);
        actix_web::error::ErrorInternalServerError("Database connection error")
    })?;

    let stmt = client
        .prepare(
            "SELECT 
            COALESCE(usage, 0) as total_usage,
            usage_limit,
            email,
            key
        FROM api_users
        WHERE user_id = $1 AND key = $2",
        )
        .await
        .map_err(|e| {
            eprintln!("Error preparing statement: {:?}", e);
            actix_web::error::ErrorInternalServerError("Database query error")
        })?;

    let row = client
        .query_one(&stmt, &[&user_id, &api_key])
        .await
        .map_err(|e| {
            eprintln!("Error executing query: {:?}", e);
            actix_web::error::ErrorInternalServerError("Database query error")
        })?;

    let total_usage: i32 = row.get("total_usage");
    let usage_limit: i32 = row.get("usage_limit");
    let email: String = row.get("email");
    let key: String = row.get("key");

    let usage_percentage = if usage_limit > 0 {
        (total_usage as f64 / usage_limit as f64) * 100.0
    } else {
        0.0
    };

    let usage_response = UsageResponse {
        email,
        key,
        total_usage,
        usage_limit,
        usage_percentage,
    };

    Ok(HttpResponse::Ok().json(usage_response))
}