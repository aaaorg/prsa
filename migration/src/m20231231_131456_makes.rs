use std::borrow::BorrowMut;

use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Makes::Table)
                    .col(pk_auto(Makes::Id).borrow_mut())
                    .col(string_uniq(Makes::Name).borrow_mut())
                    .col(bool(Makes::Active).borrow_mut())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Makes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Makes {
    Table,
    Id,
    Name,
    Active,
}
