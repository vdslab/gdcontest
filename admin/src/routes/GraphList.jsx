import { Form, Link, useLoaderData } from "react-router-dom";
import { callApi } from "../callApi";

export async function loader({ params }) {
  const contest = await callApi(`/contests/${params.contestName}`);
  const graphs = await callApi(`/contests/${params.contestName}/graphs`);
  return { contest, graphs };
}

export async function action({ request }) {
  const formData = await request.formData();
  const contestName = formData.get("contestName");
  const graphName = formData.get("graphName");
  const graph = JSON.parse(await formData.get("content").text());
  const unitEdgeLength = +formData.get("unitEdgeLength") || 0;
  if (unitEdgeLength) {
    for (const link of graph.links) {
      link.weight = unitEdgeLength;
    }
  }
  await callApi(`/contests/${contestName}/graphs/${graphName}`, {
    method: "PUT",
    body: JSON.stringify({
      nodes: graph.nodes.map((node) => ({ id: `${node.id}` })),
      links: graph.links.map((link) => ({
        source: `${link.source}`,
        target: `${link.target}`,
        weight: +link.weight,
      })),
    }),
    headers: {
      "Content-Type": "application/json",
    },
  });
  return null;
}

export default function GraphList() {
  const { contest, graphs } = useLoaderData();
  return (
    <>
      <section className="section">
        <nav className="breadcrumb">
          <ul>
            <li>
              <Link to="/">Top</Link>
            </li>
            <li className="is-active">
              <a>{contest.contest_name}</a>
            </li>
          </ul>
        </nav>
      </section>
      <section className="section">
        <h3 className="title">New Graph</h3>
        <Form method="post" encType="multipart/form-data">
          <div className="field">
            <label className="label">Contest Name</label>
            <div className="control">
              <input
                className="input"
                type="text"
                name="contestName"
                value={contest.contest_name}
                readOnly
              />
            </div>
          </div>
          <div className="field">
            <label className="label">Graph Name</label>
            <div className="control">
              <input className="input" type="text" name="graphName" required />
            </div>
          </div>
          <div className="field">
            <label className="label">Unit Edge Length</label>
            <div className="control">
              <input
                className="input"
                type="number"
                min="0"
                step="1"
                name="unitEdgeLength"
              />
            </div>
          </div>
          <div className="field">
            <label className="label">Content</label>
            <div className="control">
              <div className="file">
                <label className="file-label">
                  <input
                    className="file-input"
                    type="file"
                    accept=".json"
                    name="content"
                    required
                  />
                  <span className="file-cta">
                    <span className="file-label">Choose a file…</span>
                  </span>
                </label>
              </div>
            </div>
          </div>
          <div className="field">
            <div className="control">
              <button type="submit" className="button is-primary">
                Create
              </button>
            </div>
          </div>
        </Form>
      </section>
      <section className="section">
        <h3 className="title">Graph List</h3>
        <table className="table is-bordered is-fullwidth">
          <thead>
            <tr>
              <th className="is-3">Graph Name</th>
              <th className="is-3">Action</th>
            </tr>
          </thead>
          <tbody>
            {graphs.map((graph) => {
              return (
                <tr key={graph.graph_name}>
                  <td>{graph.graph_name}</td>
                  <td>
                    <div className="buttons">
                      <Link
                        to={`/contests/${graph.contest_name}/graphs/${graph.graph_name}/detail`}
                        className="button is-small"
                      >
                        Edit
                      </Link>
                    </div>
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </section>
    </>
  );
}
