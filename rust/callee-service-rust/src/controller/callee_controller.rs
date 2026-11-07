use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::routing::{get, post};
use axum::{Json, Router, middleware};
use serde::Deserialize;
use utoipa::{IntoParams, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

use crate::controller::dto::callee::Callee;
use crate::extensions::exception_handler::ServiceError;
use crate::extensions::http_interceptor;
use crate::logic::callee_logic::CalleeLogic;

pub struct CalleeController {
    callee_logic: Arc<CalleeLogic>,
}

impl CalleeController {
    pub fn new(callee_logic: Arc<CalleeLogic>) -> Self {
        Self { callee_logic }
    }

    /// GET /callees/sayMyName?name=
    pub fn say_my_name(&self, name: Option<String>) -> Result<Callee, ServiceError> {
        let name = name.ok_or_else(|| ServiceError::IllegalArgument("name must not be null".into()))?;
        Ok(self.callee_logic.say_my_name(&name))
    }

    /// GET /callees/sayMyOtherName/{name}
    pub fn say_my_other_name(&self, name: &str) -> Callee {
        self.callee_logic.say_my_other_name(name)
    }

    /// POST /callees/save
    pub fn save(&self, callee: Callee) -> Callee {
        self.callee_logic.save(callee)
    }

    pub fn routes(self: Arc<Self>) -> Router {
        Router::new()
            .route("/callees/sayMyName", get(say_my_name))
            .route("/callees/sayMyOtherName/{name}", get(say_my_other_name))
            .route("/callees/save", post(save))
            .layer(middleware::from_fn(http_interceptor::intercept))
            .with_state(self)
            .merge(SwaggerUi::new("/q/swagger-ui").url("/q/openapi", ApiDoc::openapi()))
    }
}

#[derive(OpenApi)]
#[openapi(paths(say_my_name, say_my_other_name, save), components(schemas(Callee)))]
struct ApiDoc;

#[derive(Deserialize, IntoParams)]
struct NameQuery {
    name: Option<String>,
}

#[utoipa::path(
    get,
    path = "/callees/sayMyName",
    params(NameQuery),
    responses(
        (status = 200, body = Callee),
        (status = 412, description = "name is missing", body = String)
    )
)]
async fn say_my_name(
    State(controller): State<Arc<CalleeController>>,
    Query(query): Query<NameQuery>,
) -> Result<Json<Callee>, ServiceError> {
    controller.say_my_name(query.name).map(Json)
}

#[utoipa::path(
    get,
    path = "/callees/sayMyOtherName/{name}",
    params(("name" = String, Path)),
    responses((status = 200, body = Callee))
)]
async fn say_my_other_name(
    State(controller): State<Arc<CalleeController>>,
    Path(name): Path<String>,
) -> Json<Callee> {
    Json(controller.say_my_other_name(&name))
}

#[utoipa::path(
    post,
    path = "/callees/save",
    request_body = Callee,
    responses((status = 200, body = Callee))
)]
async fn save(
    State(controller): State<Arc<CalleeController>>,
    Json(callee): Json<Callee>,
) -> Json<Callee> {
    Json(controller.save(callee))
}

#[cfg(test)]
mod tests;
