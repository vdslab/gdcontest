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
  JOIN users ON submissions.user_id = users.user_id
WHERE id = $1
