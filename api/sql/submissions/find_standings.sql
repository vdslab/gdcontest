SELECT 
  contest_name,
  graph_name,
  user_id,
  MIN((metrics->>'stress')::FLOAT) AS score
FROM submissions
WHERE contest_name = $1
  AND graph_name = $2
  AND metrics->>'stress' IS NOT NULL
GROUP BY contest_name, graph_name, user_id
ORDER BY score ASC
