INSERT INTO graphs (contest_name, graph_name, content, distance)
  VALUES ($1, $2, $3, $4)
ON CONFLICT (contest_name, graph_name)
DO UPDATE SET content=$3, distance=$4, updated_at=CURRENT_TIMESTAMP
RETURNING
  contest_name,
  graph_name,
  created_at,
  updated_at
