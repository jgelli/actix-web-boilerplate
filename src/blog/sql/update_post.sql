UPDATE testing.blog_posts
SET title = $2, content = $3, feature_image = $4, slug = $5, updated_at = CURRENT_TIMESTAMP
WHERE id = $1
RETURNING *;
