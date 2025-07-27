#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use serde::{ Deserialize, Serialize };
use sea_orm::{ sea_query::Order, QueryOrder };
use axum::debug_handler;

use crate::{ models::_entities::users::{ Column, Entity } };

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
  pub url: String,
  pub short_url: String,
}

#[debug_handler]
pub async fn index(State(ctx): State<AppContext>) -> Result<Response> {
  let users = Entity::find()
    .order_by(Column::Id, Order::Desc)
    .all(&ctx.db).await?;
  format::json(users)
}

pub fn routes() -> Routes {
  Routes::new().prefix("/api/users").add("/", get(index))
}
