SELECT
  content AS "content!:sqlx::types::Json<SubmissionData>"
FROM submissions
WHERE id = $1
