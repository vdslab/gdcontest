SELECT 
  contest_name,
  graph_name
FROM graphs
WHERE contest_name = $1
  AND graph_name = $2
