SELECT
  id,
  contest_name,
  graph_name,
  submissions.user_id,
  users.name AS "user_name!:Option<String>",
  users.nickname AS "user_nickname!:Option<String>",
  (metrics->>'stress')::FLOAT AS score,
  submissions.created_at,
  submissions.updated_at
FROM submissions
  LEFT JOIN users ON submissions.user_id = users.user_id
WHERE
  contest_name = $1
  AND graph_name = $2
  AND 0 = (
    SELECT count(*)
    FROM submissions AS sub
    WHERE sub.contest_name = submissions.contest_name
      AND sub.graph_name = submissions.graph_name
      AND sub.user_id = submissions.user_id
      AND sub.metrics->>'stress' IS NOT NULL
      AND (
        (sub.metrics->>'stress')::FLOAT < (submissions.metrics->>'stress')::FLOAT
        OR (
          (sub.metrics->>'stress')::FLOAT = (submissions.metrics->>'stress')::FLOAT
          AND sub.created_at < submissions.created_at
        )
      )
  )
ORDER BY score, created_at
