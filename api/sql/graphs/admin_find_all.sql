SELECT 
  contest_name,
  graph_name,
  created_at,
  updated_at
FROM graphs
WHERE contest_name = $1
