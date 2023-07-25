INSERT INTO graphs (contest_name, graph_name, content, distance)
  VALUES ($1, $2, $3, $4)
ON CONFLICT (contest_name, graph_name)
DO UPDATE SET content=$3, distance=$4
RETURNING
  contest_name,
  graph_name,
  content AS "content!:sqlx::types::Json<GraphData>",
  distance AS "distance!:sqlx::types::Json<Vec<Vec<f64>>>"
