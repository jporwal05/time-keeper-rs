#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::{
    controller::middleware::{self},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::_entities::availabilities::{ActiveModel, Entity, Model};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub total_availability: Option<i32>,
    pub available_from: Option<DateTime>,
    pub available_till: Option<DateTime>,
    pub project_id: i32,
    pub user_id: Option<uuid::Uuid>,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.total_availability = Set(self.total_availability.clone());
        item.available_from = Set(self.available_from.clone());
        item.available_till = Set(self.available_till.clone());
        item.project_id = Set(self.project_id.clone());
        item.user_id = Set(self.user_id.clone().unwrap());
    }

    fn update_with_auth(&self, auth: middleware::auth::JWT, item: &mut ActiveModel) {
        item.total_availability = Set(self.total_availability.clone());
        item.available_from = Set(self.available_from.clone());
        item.available_till = Set(self.available_till.clone());
        item.project_id = Set(self.project_id.clone());
        item.user_id = Set(auth.claims.pid.parse::<Uuid>().unwrap());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

pub async fn list(State(ctx): State<AppContext>) -> Result<Json<Vec<Model>>> {
    format::json(Entity::find().all(&ctx.db).await?)
}

pub async fn add(
    State(ctx): State<AppContext>,
    auth: middleware::auth::JWT,
    Json(params): Json<Params>,
) -> Result<Json<Model>> {
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update_with_auth(auth, &mut item);
    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Json<Model>> {
    let item = load_item(&ctx, id).await?;
    let mut item = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await?;
    format::json(item)
}

pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<()> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Json<Model>> {
    format::json(load_item(&ctx, id).await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("availabilities")
        .add("/", get(list))
        .add("/", post(add))
        .add("/:id", get(get_one))
        .add("/:id", delete(remove))
        .add("/:id", post(update))
}
