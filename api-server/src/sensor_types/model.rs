use crate::error_handler::CustomError;
use crate::schema::sensor_types;
use crate::sensors::SensorsModel;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_with::skip_serializing_none;
use uuid::Uuid;
use crate::db::DbConnection;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Identifiable)]
pub struct SensorType {
    pub id: Uuid,
    pub label: String,
    pub description: String,
    pub sensors: Option<Vec<SensorsModel>>
}

#[derive(AsChangeset, Associations, Clone, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "sensor_types"]
pub struct SensorTypesModel {
    pub id: Uuid,
    pub label: String,
    pub description: String
}

impl SensorTypesModel {
    pub fn as_hash(conn: &DbConnection) -> Result<HashMap<String, SensorType>, CustomError> {
        let sensor_types = sensor_types::table.load::<SensorTypesModel>(conn)?;
        let mut hash = HashMap::new();
        for sensor_type in sensor_types {
            hash.insert(sensor_type.id.to_string(), SensorType {
                id: sensor_type.id,
                label: sensor_type.label,
                description: sensor_type.description,
                sensors: None
            });
        }
        Ok(hash)
    }

    pub fn find_all(conn: &DbConnection) -> Result<Vec<SensorType>, CustomError> {
        let sensor_types = sensor_types::table.load::<SensorTypesModel>(conn)?;
        let sensor_types = sensor_types.into_iter().map(|sensor_type| {
            SensorType {
                id: sensor_type.id,
                label: sensor_type.label,
                description: sensor_type.description,
                sensors: None
            }
        }).collect();
        Ok(sensor_types)
    }

    pub fn find(id: Uuid, conn: &DbConnection) -> Result<SensorType, CustomError> {
        let sensor_type: SensorTypesModel = sensor_types::table.filter(sensor_types::id.eq(id)).first(conn)?;
        let sensors: Vec<SensorsModel> = SensorsModel::belonging_to(&sensor_type).load(conn)?;
        Ok(SensorType {
            id: sensor_type.id,
            label: sensor_type.label,
            description: sensor_type.description,
            sensors: Some(sensors)
        })
    }
}
