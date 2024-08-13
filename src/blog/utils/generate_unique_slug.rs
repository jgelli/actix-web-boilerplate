use slug::slugify;
use tokio_postgres::Client;

pub async fn generate_unique_slug(
    client: &Client,
    title: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut slug = slugify(title);
    let mut counter = 1;

    while slug_exists(client, &slug).await? {
        slug = format!("{}-{}", slugify(title), counter);
        counter += 1;
    }

    Ok(slug)
}

async fn slug_exists(client: &Client, slug: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let stmt = client
        .prepare("SELECT COUNT(*) FROM testing.blog_posts WHERE slug = $1")
        .await?;
    let rows = client.query_one(&stmt, &[&slug]).await?;
    let count: i64 = rows.get(0);

    Ok(count > 0)
}
