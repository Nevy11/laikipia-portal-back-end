use diesel::prelude::*;

use crate::connecting::connection_establish::establish_connection;
use crate::{models::Curriculum, schema::curriculum};

pub fn get_units() -> Vec<Curriculum> {
    let connection = &mut establish_connection();
    let curriculum_data: Vec<Curriculum> =
        curriculum::dsl::curriculum::load::<Curriculum>(self::curriculum::table, connection)
            .expect("failed to load data from the curriculum table");

    println!("{:?}", curriculum_data);
    curriculum_data
}
