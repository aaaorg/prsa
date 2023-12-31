use serde::{Deserialize, Serialize};

use crate::models::_entities::{models, printers};

#[derive(Debug, Deserialize, Serialize)]
pub struct PrinterCupsParams {
    pub name: String,
    pub driver: String,
    pub ip: String,
    pub port: i32,
}

impl PrinterCupsParams {
    #[must_use]
    pub fn new(printer: &printers::Model, model: &models::Model) -> Self {
        Self {
            name: printer.name.clone(),
            driver: model.driver.clone(),
            ip: printer.ip.clone(),
            port: printer.port,
        }
    }
}
