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
                table_auto(Printers::Table)
                    .col(pk_auto(Printers::Id).borrow_mut())
                    .col(string_uniq(Printers::Name).borrow_mut())
                    .col(string_uniq(Printers::Ip).borrow_mut())
                    .col(integer(Printers::Port).borrow_mut())
                    .col(integer(Printers::ModelId).borrow_mut())
                    .col(integer(Printers::LocationId).borrow_mut())
                    .col(bool(Printers::Active).borrow_mut())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-printers-models")
                            .from(Printers::Table, Printers::ModelId)
                            .to(Models::Table, Models::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-printers-locations")
                            .from(Printers::Table, Printers::LocationId)
                            .to(Locations::Table, Locations::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Printers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Printers {
    Table,
    Id,
    Name,
    Ip,
    Port,
    ModelId,
    LocationId,
    Active,
}

#[derive(DeriveIden)]
enum Models {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum Locations {
    Table,
    Id,
}
