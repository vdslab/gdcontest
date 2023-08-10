INSERT INTO users (user_id, name, nickname, email)
  VALUES ($1, $2, $3, $4)
ON CONFLICT (user_id)
DO UPDATE SET name=$2, nickname=$3, email=$4, updated_at=CURRENT_TIMESTAMP
RETURNING
  user_id,
  name,
  nickname,
  email,
  created_at,
  updated_at
