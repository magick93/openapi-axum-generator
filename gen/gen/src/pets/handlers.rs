// Base API template
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use axum_extra::routing::TypedPath;


/// Pet model
#[derive(Debug, Serialize, Deserialize, ToSchema, TypedPath)]
#[typed_path("/pet")]
pub struct Pet {
    
    /// id field
    pub id: i64,
    
    /// name field
    pub name: String,
    
    /// tag field
    pub tag: Option<String>,
    
}

/// Pets model
#[derive(Debug, Serialize, Deserialize, ToSchema, TypedPath)]
#[typed_path("/pets")]
pub struct Pets {
    
}

/// Error model
#[derive(Debug, Serialize, Deserialize, ToSchema, TypedPath)]
#[typed_path("/error")]
pub struct Error {
    
    /// code field
    pub code: i64,
    
    /// message field
    pub message: String,
    
}


// Base API template
use axum::{
    Router,
    routing::{get, post, put, delete},
};
use super::handlers::*;

pub fn create_router() -> Router {
    Router::new()
    .route("/pets", get(handle_get_pets))
    .route("/pets", post(handle_post_pets))
    .route("/pets/{petId}", get(handle_get_pets_petid))
    
}

// Base API template
use axum::{ extract::{Path, Query, Json}, response::Json as JsonResponse, http::StatusCode };
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;


#[utoipa::path(GET, path = "/pets",  tag = "Default",   params(("limit", i32),), 
    responses( (status = 200, description = "A paged array of pets", body = Pet),
    )
)]
pub async fn handle_get_pets(
        limit: Option<i32>, 
    
) -> Result<JsonResponse<Pet>, StatusCode> {
    // TODO: Implement handle_get_pets handler logic
    Ok(JsonResponse(Pet::default()))
}

#[utoipa::path(POST, path = "/pets",  tag = "Default",  
    responses( (status = 201, description = "Null response", body = ()),
    )
)]
pub async fn handle_post_pets(
) -> Result<JsonResponse<()>, StatusCode> {
    // TODO: Implement handle_post_pets handler logic
    Ok(JsonResponse(()))
}

#[utoipa::path(GET, path = "/pets/{petId}",  tag = "Pets",   params(("petId", String),), 
    responses( (status = 200, description = "Expected response to a valid request", body = Pet),
    )
)]
pub async fn handle_get_pets_petid(
        petId: String, 
    
) -> Result<JsonResponse<Pet>, StatusCode> {
    // TODO: Implement handle_get_pets_petid handler logic
    Ok(JsonResponse(Pet::default()))
}




#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use serde_json::{json, Value};
    use tower::ServiceExt;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_example_endpoint() -> Result<(), Box<dyn std::error::Error>> {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/example")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!({ "data": "example" }));
    }
}