#![deny(
     missing_docs,
     non_camel_case_types,
     non_snake_case,
     path_statements,
     trivial_casts,
     trivial_numeric_casts,
     unsafe_code,
     unstable_features,
     unused_allocation,
     unused_import_braces,
     unused_imports,
     unused_must_use,
     unused_mut,
     while_true,
 )]

//! # Hydrazine
//! ### An opinionated admin interface
//!
//! To put it simple, Hydrazine is meant to make your task easy to setup a quick admin interface
//! for your web projects. It is written for the [Rocket][rocket] framework.
//!
//! [rocket]: https://rocket.rs
//!

#[cfg(test)] extern crate dotenv;

#[macro_use] extern crate diesel;
#[macro_use] extern crate error_chain;
extern crate itertools;

mod schema;
mod error;


#[cfg(test)]
mod tests {
    use super::schema::*;
    use diesel::pg::PgConnection;

    pub fn establish_connection() -> PgConnection {
        use std::env;
        use dotenv::dotenv;
        use diesel::prelude::*;
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
    }

    #[test]
    fn check_users_tables() {
        let conn = establish_connection();
        let tables : Vec<Table> = Schema::load(&conn, &["public"], &["users"]).expect("Load schema tables");
        assert!(tables.len() == 1);
        assert!(tables[0].name == "users");
    }
}
