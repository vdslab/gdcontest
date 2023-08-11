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
WHERE contest_name = $1
  AND graph_name = $2
ORDER BY score, created_at
