INSERT INTO submissions (contest_name, graph_name, user_id, content, metrics)
  VALUES ($1, $2, $3, $4, $5)
RETURNING
  id,
  contest_name,
  graph_name,
  user_id,
  (metrics->>'stress')::FLOAT AS score
