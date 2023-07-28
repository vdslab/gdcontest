export async function fetchContests() {
  const response = await fetch(`${process.env.API_ORIGIN}/contests`);
  return response.json();
}

export async function fetchContest(contestName) {
  const response = await fetch(
    `${process.env.API_ORIGIN}/contests/${contestName}`,
  );
  return response.json();
}

export async function fetchGraphs(contestName) {
  const response = await fetch(
    `${process.env.API_ORIGIN}/contests/${contestName}/graphs`,
  );
  return response.json();
}

export async function fetchGraph(contestName, graphName) {
  const response = await fetch(
    `${process.env.API_ORIGIN}/contests/${contestName}/graphs/${graphName}`,
  );
  return response.json();
}

export async function fetchGraphContent(contestName, graphName) {
  const response = await fetch(
    `${process.env.API_ORIGIN}/contests/${contestName}/graphs/${graphName}/content`,
  );
  return response.json();
}

export async function fetchStanginsSubmissions(contestName, graphName) {
  const response = await fetch(
    `${process.env.API_ORIGIN}/contests/${contestName}/graphs/${graphName}/standings`,
  );
  return response.json();
}
