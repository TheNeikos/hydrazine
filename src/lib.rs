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
    fn check_all_tables() {
        let conn = establish_connection();
        let _tables : Vec<Table> = Schema::load(&conn, &["public"], &["users"]).expect("Load schema tables");
        println!("{:#?}", _tables);
    }
}
