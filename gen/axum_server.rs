// Base API template
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use axum_extra::routing::TypedPath;


/// dataSetList model
#[derive(Debug, Serialize, Deserialize, ToSchema, TypedPath)]
#[typed_path("/datasetlist")]
pub struct dataSetList {
    
    /// total field
    pub total: Option<i64>,
    
    /// apis field
    pub apis: Option<Vec<HashMap<String, Value>>>,
    
}


// Base API template
use axum::{
    Router,
    routing::{get, post, put, delete},
};
use super::handlers::*;

pub fn create_router() -> Router {
    Router::new()
    
}

// Base API template
use axum::{ extract::{Path, Query, Json}, response::Json as JsonResponse, http::StatusCode };
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;





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