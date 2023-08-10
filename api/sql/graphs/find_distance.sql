SELECT 
  distance
FROM graphs
WHERE contest_name = $1
  AND graph_name = $2
