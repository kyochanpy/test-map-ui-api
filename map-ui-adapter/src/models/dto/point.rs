use derive_new::new;

#[derive(new)]
pub struct PointDto {
    pub point_type: String,
    pub name: String,
    pub address: String,
    pub description: String,
}
