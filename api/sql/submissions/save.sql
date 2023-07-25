INSERT INTO submissions (contest_name, graph_name, user_id, content)
  VALUES ($1, $2, $3, $4)
RETURNING
  id,
  contest_name,
  graph_name,
  user_id
