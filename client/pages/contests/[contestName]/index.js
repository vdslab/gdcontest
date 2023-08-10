import { fetchContest, fetchGraphs } from "@/api";
import { useRouter } from "next/router";
import Link from "next/link";

export async function getStaticProps({ params }) {
  const { contestName } = params;
  const contest = await fetchContest(contestName);
  const graphs = await fetchGraphs(contestName);
  return {
    props: {
      contest,
      graphs,
    },
    revalidate: 10,
  };
}

export async function getStaticPaths() {
  return {
    paths: [],
    fallback: "blocking",
  };
}

export default function ContestDetailPage({ contest, graphs }) {
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
                <th>Graph Name</th>
              </tr>
            </thead>
            <tbody>
              {graphs.map((graph) => {
                return (
                  <tr key={graph.graph_name}>
                    <td>
                      <Link
                        href={`/contests/${graph.contest_name}/${graph.graph_name}`}
                      >
                        {graph.graph_name}
                      </Link>
                    </td>
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
