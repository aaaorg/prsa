#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;
mod m20231103_114510_notes;

mod m20231231_131456_makes;
mod m20231231_131936_models;
mod m20231231_132449_locations;
mod m20231231_132951_printers;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20231103_114510_notes::Migration),
            Box::new(m20231231_131456_makes::Migration),
            Box::new(m20231231_131936_models::Migration),
            Box::new(m20231231_132449_locations::Migration),
            Box::new(m20231231_132951_printers::Migration),
        ]
    }
}
