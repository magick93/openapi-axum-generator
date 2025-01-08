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
    .route("/", get(handle_get_root))
    .route("/{dataset}/{version}/fields", get(handle_get_datasets_version_fields))
    .route("/{dataset}/{version}/records", post(handle_post_datasets_version_records))
    
}

// Base API template
use axum::{ extract::{Path, Query, Json}, response::Json as JsonResponse, http::StatusCode };
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;


#[utoipa::path(GET, path = "/",  tag = "metadata",  
    responses( (status = 200, description = "Returns a list of data sets", body = Pet),
    )
)]
pub async fn handle_get_root(
) -> Result<JsonResponse<Pet>, StatusCode> {
    // TODO: Implement handle_get_root handler logic
    Ok(JsonResponse(Pet::default()))
}

#[utoipa::path(GET, path = "/{dataset}/{version}/fields",  tag = "metadata",   params(("dataset", String),("version", String),), 
    responses( (status = 200, description = "The dataset API for the given version is found and it is accessible to consume.", body = Pet), (status = 404, description = "The combination of dataset name and version is not found in the system or it is not published yet to be consumed by public.", body = Pet),
    )
)]
pub async fn handle_get_datasets_version_fields(
        dataset: String, 
        version: String, 
    
) -> Result<JsonResponse<Pet>, StatusCode> {
    // TODO: Implement handle_get_datasets_version_fields handler logic
    Ok(JsonResponse(Pet::default()))
}

#[utoipa::path(POST, path = "/{dataset}/{version}/records",  tag = "search",   params(("version", String),("dataset", String),), 
    responses( (status = 200, description = "successful operation", body = Pet), (status = 404, description = "No matching record found for the given criteria.", body = ()),
    )
)]
pub async fn handle_post_datasets_version_records(
        version: String, 
        dataset: String, 
    
) -> Result<JsonResponse<Pet>, StatusCode> {
    // TODO: Implement handle_post_datasets_version_records handler logic
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