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
                table_auto(Locations::Table)
                    .col(pk_auto(Locations::Id).borrow_mut())
                    .col(string_uniq(Locations::Name).borrow_mut())
                    .col(integer_uniq(Locations::Code).borrow_mut())
                    .col(integer_uniq(Locations::IpCode).borrow_mut())
                    .col(bool(Locations::Active).borrow_mut())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Locations::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Locations {
    Table,
    Id,
    Name,
    Code,
    IpCode,
    Active,
}
