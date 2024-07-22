use derive_new::new;

#[derive(new, Debug)]
pub struct PointDto {
    pub point_type: String,
    pub name: String,
    pub address: String,
    pub description: String,
    pub latitude: f64,
    pub longitude: f64,
}
