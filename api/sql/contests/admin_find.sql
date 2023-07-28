SELECT
  contest_name,
  published,
  start_at,
  end_at,
  created_at,
  updated_at
FROM contests
WHERE contest_name = $1
