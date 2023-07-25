SELECT 
  id,
  contest_name,
  graph_name,
  user_id
FROM submissions
WHERE contest_name = $1
  AND graph_name = $2
  AND user_id = $3
