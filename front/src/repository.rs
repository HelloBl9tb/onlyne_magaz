use tokio_postgres::{NoTls, Row};
use crate::models::User;

pub struct UserRepository {
    conn: tokio_postgres::Client,
}

impl UserRepository {
    pub async fn new() -> Self {
        let (client, connection) = tokio_postgres::connect(
            "host=localhost user=myuser password=mypassword dbname=mydatabase",
            NoTls,
        )
       .await
       .unwrap();

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Self { conn: client }
    }

    pub async fn get_user(&self, id: i32) -> Option<User> {
        let row = self
           .conn
           .query_one("SELECT * FROM users WHERE id = $1", &[&id])
           .await
           .unwrap();

        row.map(|row| User {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
        })
    }
}