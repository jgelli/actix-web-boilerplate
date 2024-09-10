SELECT *
FROM testing.blog_posts
WHERE id < $1
  AND active = $2
  AND deleted_at IS NULL
  AND (
      $3::TEXT IS NULL
      OR title ILIKE '%' || $3 || '%'
    )
ORDER BY id DESC LIMIT $4;
