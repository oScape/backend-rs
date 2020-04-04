use crate::schema::drivers;

#[derive(Queryable)]
pub struct Driver {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub enabled: bool,
}

#[derive(Insertable)]
#[table_name = "drivers"]
pub struct NewDriver<'a> {
    pub firstname: &'a str,
    pub lastname: &'a str,
}
