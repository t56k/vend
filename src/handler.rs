#[get("/")]
pub fn all_items(connection: DbConn) -> Result<Json<Vec<Item>>, Status> {
    sample::repository::show_items(&connection)
        .map(|item| Json(item))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<new_item>")]
pub fn create_item(
    new_item: Json<NewItem>,
    connection: DbConn,
) -> Result<status::Created<Json<Item>>, Status> {
    println!("here 0 {}", &new_item.title);
    sample::repository::create_item(new_item.into_inner(), &connection)
        .map(|item| item_created(item))
        .map_err(|error| error_status(error))
}
#[get("/<id>")]
pub fn get_item(id: i32, connection: DbConn) -> Result<Json<Item>, Status> {
    sample::repository::get_item(id, &connection)
        .map(|item| Json(item))
        .map_err(|error| error_status(error))
}

#[put("/<id>", format = "application/json", data = "<item>")]
pub fn update_item(id: i32, item: Json<Item>, connection: DbConn) -> Result<Json<Item>, Status> {
    sample::repository::update_item(id, item.into_inner(), &connection)
        .map(|item| Json(item))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete_item(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    sample::repository::delete_item(id, &connection)
        .map(|_| status::NoContent)
        .map_err(|error| error_status(error))
}
