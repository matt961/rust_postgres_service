use super::schema::posts;

#[derive(Serialize, Deserialize)]
#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub title: String,
    pub body: String
}
