UPDATE testing.blog_posts
SET deleted_at = CURRENT_TIMESTAMP
WHERE id = $1
RETURNING *
