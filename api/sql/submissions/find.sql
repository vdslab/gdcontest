SELECT
  id,
  contest_name,
  graph_name,
  user_id,
  (metrics->>'stress')::FLOAT AS score
FROM submissions
WHERE id = $1
