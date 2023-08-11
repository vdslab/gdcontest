export async function fetchContests() {
  const response = await fetch(
    `${process.env.NEXT_PUBLIC_API_ORIGIN}/contests`,
  );
  return response.json();
}

export async function fetchContest(contestName) {
  const response = await fetch(
    `${process.env.NEXT_PUBLIC_API_ORIGIN}/contests/${contestName}`,
  );
  return response.json();
}

export async function fetchGraphs(contestName) {
  const response = await fetch(
    `${process.env.NEXT_PUBLIC_API_ORIGIN}/contests/${contestName}/graphs`,
  );
  return response.json();
}

export async function fetchGraph(contestName, graphName) {
  const response = await fetch(
    `${process.env.NEXT_PUBLIC_API_ORIGIN}/contests/${contestName}/graphs/${graphName}`,
  );
  return response.json();
}

export async function fetchGraphContent(contestName, graphName) {
  const response = await fetch(
    `${process.env.NEXT_PUBLIC_API_ORIGIN}/contests/${contestName}/graphs/${graphName}/content`,
  );
  return response.json();
}

export async function fetchStanginsSubmissions(contestName, graphName) {
  const response = await fetch(
    `${process.env.NEXT_PUBLIC_API_ORIGIN}/contests/${contestName}/graphs/${graphName}/standings`,
  );
  return response.json();
}

export async function fetchSubmission(submissionId) {
  const response = await fetch(
    `${process.env.NEXT_PUBLIC_API_ORIGIN}/submissions/${submissionId}`,
  );
  return response.json();
}

export async function postSubmission(contestName, graphName, content) {
  const response = await fetch(`/api/submissions/${contestName}/${graphName}`, {
    method: "POST",
    body: JSON.stringify(content),
    headers: {
      "Content-Type": "application/json",
    },
  });
  return response.json();
}

export async function fetchUserSubmissions(contestName, graphName, userId) {
  const response = await fetch(
    `${process.env.NEXT_PUBLIC_API_ORIGIN}/contests/${contestName}/graphs/${graphName}/users/${userId}/submissions`,
  );
  return response.json();
}
