use sqlx::SqlitePool;

pub struct VinDatabase {
    pool: SqlitePool,
}

impl VinDatabase {
    pub async fn new(path: &str) -> Self {
        let pool = SqlitePool::connect(path).await.unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS seen (
            user INTEGER NOT NULL,
            pageurl TEXT NOT NULL,
            UNIQUE (user, pageurl)
        );
        CREATE TABLE IF NOT EXISTS searches (
            user INTEGER NOT NULL,
            search TEXT NOT NULL,
            UNIQUE (user, searchurl)
        );",
        )
        .execute(&pool)
        .await
        .unwrap();

        Self { pool }
    }

    pub async fn put_seen(&mut self, user: &i64, id: &i64) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT OR IGNORE INTO seen (user, pageurl) VALUES (?, ?);")
            .bind(user)
            .bind(id)
            .execute(&self.pool)
            .await
            .map(|_| ())
    }

    pub async fn is_seen(&mut self, user: &i64, id: &i64) -> Result<(bool,), sqlx::Error> {
        sqlx::query_as("SELECT EXISTS ( SELECT 1 FROM seen WHERE user = ? AND pageurl = ?);")
            .bind(user)
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }
}
