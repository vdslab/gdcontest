SELECT 
  contest_name,
  graph_name,
  content AS "content!:sqlx::types::Json<GraphData>",
  distance AS "distance!:sqlx::types::Json<DistanceData>"
FROM graphs
WHERE contest_name = $1
  AND graph_name = $2
