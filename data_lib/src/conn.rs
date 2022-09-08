use mysql::*;

/// `GenericConnPoolWrapper` is a wrapper around a `Pool` of `GenericConnection`s.
///
/// Properties:
///
/// * `connection_pool`: This is the connection pool that we'll be using to manage our connections.
pub struct GenericConnPoolWrapper {
    connection_pool: Pool,
}

impl GenericConnPoolWrapper {
    pub fn new(url: &str) -> Self {
        let pool: Pool = Pool::new(url)
            .unwrap_or_else(|| panic!("Failed to connect to mysql url: {}", url));
        Self { connection_pool: pool }
    }

    /// Get a connection from the connection pool and return it.
    ///
    /// Returns:
    ///
    /// A PooledConn
    pub fn get_connection(&self) -> PooledConn {
        self.connection_pool.get_conn().expect("Failed to get connection")
    }
}

#[cfg(test)]
mod test {
    use mysql::{params, PooledConn};
    use mysql::prelude::Queryable;
    use crate::conn::GenericConnPoolWrapper;

    #[test]
    fn check_conn() {
        #[derive(Debug)]
        struct TheTest { val: String }
        impl TheTest { pub fn val(&self) -> &str { &self.val } }
        let conn_pool_wrapper: GenericConnPoolWrapper = GenericConnPoolWrapper
        ::new("mysql://rust:ThePassword@localhost:3306/rust_web_test");
        let mut pooled_conn: PooledConn = conn_pool_wrapper.get_connection();
        pooled_conn.query_drop(
            r"CREATE TEMPORARY TABLE testing (
                     val TEXT
           )").expect("Failed to create temporary table.");
        let mut tests: Vec<TheTest> = Vec::new();
        for i in 0..5 {
           tests.push(TheTest { val: format!("Hey:{}", i) });
        }
        pooled_conn.exec_batch(
            r"INSERT INTO testing (val) VALUES (:val)",
            tests.iter().map(|test: &TheTest| params! {
                "val" => test.val()
            })
        ).expect("Failed to insert.");
        let selected_tests: Vec<TheTest> = pooled_conn
            .query_map("SELECT val FROM testing",
            |val| { TheTest { val } })
            .expect("Select failed");
        dbg!(selected_tests);
    }
}