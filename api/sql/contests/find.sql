SELECT
  contest_name,
  start_at,
  end_at
FROM contests
WHERE contest_name = $1
  AND published = TRUE
