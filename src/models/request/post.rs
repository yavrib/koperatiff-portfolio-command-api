use models::schema::{posts};

#[derive(Insertable, AsChangeset)]
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
#[table_name="posts"]
pub struct Post {
    pub id: Option<i32>,
    pub title: String,
    pub post_type: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub published: bool
}
