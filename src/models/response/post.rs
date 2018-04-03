use std::time::SystemTime;

#[derive(Queryable)]
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub post_type: String,
    pub description: String,
    pub image_url: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub published: bool
}
