import { fetchContest, fetchGraphs, fetchStanginsSubmissions } from "@/api";
import Link from "next/link";
import { calculatePoint } from "@/standings";
import { useUser } from "@auth0/nextjs-auth0/client";

export async function getStaticProps({ params }) {
  const { contestName } = params;
  const contest = await fetchContest(contestName);
  const graphs = await fetchGraphs(contestName);
  const standings = await Promise.all(
    graphs.map((graph) =>
      fetchStanginsSubmissions(contestName, graph.graph_name),
    ),
  );
  const userPoints = {};
  for (const submissions of standings) {
    const points = calculatePoint(submissions);
    submissions.forEach(({ user_id, user_name }, i) => {
      if (!userPoints[user_id]) {
        userPoints[user_id] = { point: 0, count: 0 };
      }
      userPoints[user_id].name = user_name || user_id;
      userPoints[user_id].point += points[i];
      userPoints[user_id].count += 1;
    });
  }
  const userStandings = Object.entries(userPoints).map(
    ([user_id, { name, point, count }]) => ({ user_id, name, point, count }),
  );
  userStandings.sort((a, b) => b.point - a.point);
  return {
    props: {
      contest,
      graphs,
      standings,
      userStandings,
    },
    revalidate: 600,
  };
}

export async function getStaticPaths() {
  return {
    paths: [],
    fallback: "blocking",
  };
}

export default function ContestDetailPage({
  contest,
  graphs,
  standings,
  userStandings,
}) {
  const { user } = useUser();
  return (
    <>
      <div className="block">
        <nav className="breadcrumb" aria-label="breadcrumbs">
          <ul>
            <li className="is-active">
              <a>{contest.contest_name}</a>
            </li>
          </ul>
        </nav>
      </div>
      <div className="block">
        <h3 className="title">Contest Detail</h3>
        <div className="field">
          <label className="label">Contest Name</label>
          <div className="control">
            <input className="input" value={contest.contest_name} readOnly />
          </div>
        </div>
        <div className="field">
          <label className="label">Start</label>
          <div className="control">
            <input
              className="input"
              type="datetime-local"
              value={contest.start_at}
              readOnly
            />
          </div>
        </div>
        <div className="field">
          <label className="label">End</label>
          <div className="control">
            <input
              className="input"
              type="datetime-local"
              value={contest.end_at}
              readOnly
            />
          </div>
        </div>
      </div>
      <div className="block">
        <h3 className="title">Graphs</h3>
        <div className="table-container">
          <table className="table is-bordered is-fullwidth">
            <thead>
              <tr>
                <th className="has-text-centered" style={{ width: "50%" }}>
                  Graph Name
                </th>
                <th className="has-text-centered" style={{ width: "50%" }}>
                  Submitted Users
                </th>
              </tr>
            </thead>
            <tbody>
              {graphs.map((graph, i) => {
                return (
                  <tr
                    key={graph.graph_name}
                    className={
                      user &&
                      standings[i].some(({ user_id }) => user_id === user.sub)
                        ? "is-selected"
                        : ""
                    }
                  >
                    <td>
                      <Link
                        href={`/contests/${graph.contest_name}/${graph.graph_name}`}
                      >
                        {graph.graph_name}
                      </Link>
                    </td>
                    <td className="has-text-right">{standings[i].length}</td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      </div>
      <div className="block">
        <h3 className="title">Standings</h3>
        <div className="table-container">
          <table className="table is-bordered is-fullwidth">
            <thead>
              <tr>
                <th className="has-text-centered" style={{ width: "25%" }}>
                  Position
                </th>
                <th className="has-text-centered" style={{ width: "25%" }}>
                  User
                </th>
                <th className="has-text-centered" style={{ width: "25%" }}>
                  Submitted Graphs
                </th>
                <th className="has-text-centered" style={{ width: "25%" }}>
                  Total Point
                </th>
              </tr>
            </thead>
            <tbody>
              {userStandings.map(({ user_id, name, point, count }, i) => {
                return (
                  <tr
                    key={i}
                    className={
                      user && user_id === user.sub ? "is-selected" : ""
                    }
                  >
                    <td className="has-text-left">{i + 1}</td>
                    <td className="has-text-left">{name}</td>
                    <td className="has-text-right">
                      {count} / {graphs.length}
                    </td>
                    <td className="has-text-right">{point}</td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      </div>
    </>
  );
}
