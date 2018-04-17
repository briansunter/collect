use diesel::pg::PgConnection;
use data::Data;
use data::NewData;
use diesel;
use diesel::prelude::*;

pub trait DataDao {
    fn create_data(&self, content: &str) -> Data;
    fn get_data(&self) -> Vec<Data>;
}

pub struct DataDaoPG<'a> {
    conn: &'a PgConnection
}

impl<'a> DataDaoPG<'a> {
    pub fn new(conn: &'a PgConnection) -> Self {
        DataDaoPG {conn: conn}
    }
}

impl<'a> DataDao for DataDaoPG<'a> {
    fn get_data(&self) -> Vec<Data> {
        use schema::data::dsl::*;

        return data
        .limit(10)
        .load::<Data>(self.conn)
        .expect("Error!");
    }

    fn create_data(&self, content: &str) -> Data {
        use schema::data;
        
        let new_data = &NewData {
            content: content
        };

        diesel::insert_into(data::table)
        .values(new_data)
        .get_result(self.conn)
        .expect("Error saving new post")
    }
}
