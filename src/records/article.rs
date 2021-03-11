use serde::{Deserialize, Serialize};
type Query<'q> = sqlx::query::Query<'q, sqlx::Sqlite, sqlx::sqlite::SqliteArguments<'q>>;
type QueryAs<'q, T> = sqlx::query::QueryAs<'q, sqlx::Sqlite, T, sqlx::sqlite::SqliteArguments<'q>>;

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct Article {
    pub id: i64,
    pub text: String,
    pub title: String,
    pub created: String,
    pub updated: String,
}

impl crate::utils::AsRoute for Article {
    fn as_route(&self) -> std::borrow::Cow<str> {
        format!("/articles/{}", self.id).into()
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct PartialArticle {
    pub text: Option<String>,
    pub title: Option<String>,
}

impl PartialArticle {
    pub fn update_by_id(&self, id: i64) -> Query {
        sqlx::query(
            "UPDATE articles (text, title, updated) VALUES (
            COALESCE($1, articles.text),
            COALESCE($2, articles.title),
            datetime('now')
          ) WHERE id = $3",
        )
        .bind(&self.text)
        .bind(&self.title)
        .bind(id)
    }

    pub fn create(&self) -> Query {
        sqlx::query(
            "INSERT INTO articles (text, title, created, updated) VALUES (
            $1, $2, DATETIME('now'), DATETIME('now')
          )",
        )
        .bind(&self.text)
        .bind(&self.title)
    }
}

impl<'q> Article {
    pub fn all() -> QueryAs<'q, Self> {
        sqlx::query_as("SELECT * FROM articles")
    }

    pub fn last_id() -> Query<'q> {
        sqlx::query("SELECT last_insert_rowid()")
    }

    pub fn find_by_id(id: i64) -> QueryAs<'q, Self> {
        sqlx::query_as("SELECT * FROM articles WHERE id = ?").bind(id)
    }

    pub fn delete_by_id(id: i64) -> Query<'q> {
        sqlx::query("DELETE FROM articles WHERE id = ?").bind(id)
    }

    // pub fn update(&self, partial: PartialArticle) -> Query {
    //     partial.update_by_id(self.id)
    // }
}
