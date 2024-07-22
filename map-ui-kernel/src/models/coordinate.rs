use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}
