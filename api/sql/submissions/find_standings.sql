WITH best_submissions AS (
  SELECT 
    contest_name,
    graph_name,
    user_id,
    MIN((metrics->>'stress')::FLOAT) AS score
  FROM submissions
  WHERE contest_name = $1
    AND graph_name = $2
    AND metrics->>'stress' IS NOT NULL
  GROUP BY contest_name, graph_name, submissions.user_id
)
SELECT 
  contest_name,
  graph_name,
  best_submissions.user_id,
  users.name AS "user_name!:Option<String>",
  users.nickname AS "user_nickname!:Option<String>",
  score
FROM best_submissions
  LEFT JOIN users ON best_submissions.user_id = users.user_id
ORDER BY score ASC
