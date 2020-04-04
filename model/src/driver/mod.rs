pub mod model;

use model::{Driver, NewDriver};
use diesel::prelude::*;
use crate::utils::establish_connection;

pub fn create_driver(firstname: &str, lastname: &str) -> Driver {
    use crate::schema::drivers;
    
    let connection = establish_connection();

    let new_driver = NewDriver {
        firstname: firstname,
        lastname: lastname,
    };

    diesel::insert_into(drivers::table)
        .values(&new_driver)
        .execute(&connection)
        .expect("Error saving new driver");
    
    drivers::table.order(drivers::id.desc()).first(&connection).unwrap()
}

pub fn get_driver(driver_id: i32) -> Driver {
    use crate::schema::drivers::dsl::{drivers, id};

    let connection = establish_connection();

    drivers.filter(id.eq(driver_id)).first(&connection).expect("Error loading driver")
}

pub fn enable_driver(driver_id: i32) {
    use crate::schema::drivers::dsl::{drivers, enabled};

    let connection = establish_connection();

    diesel::update(drivers.find(driver_id))
        .set(enabled.eq(true))
        .execute(&connection)
        .unwrap();
}

pub fn disable_driver(driver_id: i32) {
    use crate::schema::drivers::dsl::{drivers, enabled};

    let connection = establish_connection();

    diesel::update(drivers.find(driver_id))
        .set(enabled.eq(false))
        .execute(&connection)
        .unwrap();
}
