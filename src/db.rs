use redis::Commands;

pub struct ShortyLink {
    pub url: String,
}

pub struct ShortyDb {
    conn: redis::Client,
}

impl ShortyDb {
    pub fn new(conn: redis::Client) -> Self {
        ShortyDb { conn }
    }

    pub fn add_link(&mut self, name: String, link: ShortyLink) -> Result<(), String> {
        let _: () = self.conn.set(format!("link:{}", name), "howdy").unwrap();

        Ok(())
    }
}
