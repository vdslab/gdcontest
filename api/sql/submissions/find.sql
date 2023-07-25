SELECT
  id,
  contest_name,
  graph_name,
  user_id
FROM submissions
WHERE id = $1