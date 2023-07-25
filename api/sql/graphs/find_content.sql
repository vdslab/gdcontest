SELECT 
  content AS "content!:sqlx::types::Json<GraphData>"
FROM graphs
WHERE contest_name = $1
  AND graph_name = $2
