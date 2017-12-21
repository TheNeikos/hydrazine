use diesel::pg::PgConnection;
use itertools::Itertools;

use error::HResult;

#[derive(Debug, Clone)]
pub struct Column {
    typ: String,
    name: String,
}

#[derive(Debug, Clone)]
pub struct Table {
    schema: String,
    name: String,
    columns: Vec<Column>,
}

#[derive(Queryable)]
pub struct Schema {
    table_schema: String,
    table_name: String,
    column_name: String,
    udt_name: String,
}

impl Schema {
    pub fn load(conn: &PgConnection, schemas: &[&str], tables: &[&str])
        -> HResult<Vec<Table>>
    {
        use diesel::prelude::*;
        use self::columns::dsl::*;
        use self::columns::table;

        let mut schemas = columns
            .filter(table_schema.eq_any(schemas))
            .filter(table_name.eq_any(tables))
            .load::<Schema>(conn)?;
        schemas.sort_by(|s, t| (&s.table_schema, &s.table_name).cmp(&(&t.table_schema, &t.table_name)));
        Ok(schemas.into_iter()
            .group_by(|s| (s.table_schema.clone(), s.table_name.clone()))
            .into_iter()
            .map(|((schema, name), group)| {
                use schema::Table;
                Table {
                    schema: schema,
                    name: name,
                    columns: group.map(|s| {
                        use schema::Column;
                        Column {
                            typ: s.udt_name,
                            name: s.column_name,
                        }
                    }).collect(),
                }
            }).collect())
    }
}

table! {
    information_schema.columns (table_schema, table_name, column_name) {
        table_schema -> VarChar,
        table_name -> VarChar,
        column_name -> VarChar,
        udt_name -> VarChar,
    }
}

