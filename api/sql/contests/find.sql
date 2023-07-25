SELECT
  contest_name,
  is_public
FROM contests
WHERE contest_name = $1
