use schema::data;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Data {
    pub id: i32,
    pub content: String
}

#[derive(Insertable)]
#[table_name="data"]
pub struct NewData<'a> {
    pub content: &'a str,
}
