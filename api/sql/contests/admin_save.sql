INSERT INTO contests (contest_name, published, start_at, end_at)
  VALUES ($1, $2, $3, $4)
ON CONFLICT (contest_name)
DO UPDATE SET
  published=$2,
  start_at=$3,
  end_at=$4,
  updated_at=CURRENT_TIMESTAMP
RETURNING
  contest_name,
  published,
  start_at,
  end_at,
  created_at,
  updated_at
