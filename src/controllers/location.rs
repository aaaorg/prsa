#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, Condition, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::models::_entities::{
    locations::{ActiveModel, Column, Entity, Model},
    printers,
};
use crate::views::printer::PrinterCupsParams;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub name: String,
    pub code: Option<i32>,
    pub ip_code: Option<i32>,
    pub active: bool,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.name = Set(self.name.clone());
        item.code = Set(self.code.clone());
        item.ip_code = Set(self.ip_code.clone());
        item.active = Set(self.active.clone());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

pub async fn list(State(ctx): State<AppContext>) -> Result<Json<Vec<Model>>> {
    format::json(Entity::find().all(&ctx.db).await?)
}

pub async fn list_printers_in_location(
    Path(ip_code): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Json<Vec<PrinterCupsParams>>> {
    let location = Entity::find()
        .filter(Condition::all().add(Column::IpCode.eq(ip_code)))
        .all(&ctx.db)
        .await?;
    let printers = location
        .first()
        .unwrap()
        .find_related(printers::Entity)
        .all(&ctx.db)
        .await?;
    let printers = printers
        .into_iter()
        .map(|printer| PrinterCupsParams::new(&printer))
        .collect();
    format::json(printers)
}

pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Json<Model>> {
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
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
        .prefix("locations")
        .add("/", get(list))
        .add("/", post(add))
        .add("/:id", get(get_one))
        .add("/:id", delete(remove))
        .add("/:id", post(update))
        .add("/:ip_code/printers", get(list_printers_in_location))
}
