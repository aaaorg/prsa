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
                table_auto(Models::Table)
                    .col(pk_auto(Models::Id).borrow_mut())
                    .col(string_uniq(Models::Name).borrow_mut())
                    .col(string(Models::Driver).borrow_mut())
                    .col(integer(Models::MakeId).borrow_mut())
                    .col(bool(Models::Active).borrow_mut())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-models-makes")
                            .from(Models::Table, Models::MakeId)
                            .to(Makes::Table, Makes::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Models::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Models {
    Table,
    Id,
    Name,
    Driver,
    MakeId,
    Active,
}

#[derive(DeriveIden)]
enum Makes {
    Table,
    Id,
}
