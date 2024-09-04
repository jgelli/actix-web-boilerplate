SELECT $table_fields FROM testing.blog_posts WHERE slug = $1 AND deleted_at is null
