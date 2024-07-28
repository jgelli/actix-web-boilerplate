INSERT INTO testing.blog_posts (title, content, feature_image, slug, author, created_at, updated_at)
VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
RETURNING $table_fields;
