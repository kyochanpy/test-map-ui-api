use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use tracing::error;

use crate::{
    models::point::{JsonPoint, Point},
    modules::{Modules, ModulesExt},
};

pub async fn point(
    State(modules): State<Arc<Modules>>,
    Json(source): Json<Vec<JsonPoint>>,
) -> Result<impl IntoResponse, StatusCode> {
    let coordinates = source.into_iter().map(|p| p.into()).collect();
    print!("{:?}", coordinates);
    let res = modules
        .point_usecase()
        .get_point(coordinates)
        .await
        .map(|points| {
            points
                .into_iter()
                .map(|point| point.into())
                .collect::<Vec<Point>>()
        });
    res.map(|pokemons| (StatusCode::OK, Json(pokemons)))
        .inspect_err(|e| {
            error!("Unexpected error: {:?}", e);
        })
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// 上のエンドポイントに投げるリクエストの例
// curl -X POST -H "Content-Type: application/json" -d '[{"latitude": 35.681236, "longitude": 139.767125}]' http://localhost:3000/point
