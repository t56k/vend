#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "items"]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub cost: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "items"]
pub struct NewItem {
    pub name: String,
    pub description: String,
}
