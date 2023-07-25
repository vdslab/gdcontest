INSERT INTO contests (contest_name, is_public)
  VALUES ($1, $2)
ON CONFLICT (contest_name)
DO UPDATE SET is_public=$2
RETURNING
  contest_name,
  is_public
