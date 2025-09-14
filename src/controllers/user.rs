#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use sea_orm::{ sea_query::Order, QueryOrder };
use axum::debug_handler;

use crate::{ models::_entities::users::{ Column, Entity } };

#[debug_handler]
pub async fn hello(State(_ctx): State<AppContext>) -> Result<Response> {
  format::json(serde_json::json!({ "message": "Hello World" }))
}

#[debug_handler]
pub async fn users(State(ctx): State<AppContext>) -> Result<Response> {
  let users = Entity::find()
    .order_by(Column::Id, Order::Desc)
    .all(&ctx.db).await?;
  format::json(users)
}

pub fn routes() -> Routes {
  Routes::new()
    .prefix("/api")
    .add("/hello", get(hello))
    .add("/users", get(users))
}
