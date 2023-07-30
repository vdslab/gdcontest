import { Form, Link, useLoaderData } from "react-router-dom";
import { callApi } from "../callApi";

export async function loader({ params }) {
  const graph = await callApi(
    `/contests/${params.contestName}/graphs/${params.graphName}`,
  );
  const content = await callApi(
    `/contests/${params.contestName}/graphs/${params.graphName}/content`,
  );
  return { graph, content };
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
  return callApi(`/contests/${contestName}/graphs/${graphName}`, {
    method: "PUT",
    body: JSON.stringify({
      nodes: graph.nodes.map((node) => ({ id: node.id })),
      links: graph.links.map((link) => ({
        source: link.source,
        target: link.target,
        weight: link.weight,
      })),
    }),
    headers: {
      "Content-Type": "application/json",
    },
  });
}

export default function GraphDetail() {
  const { graph, content } = useLoaderData();
  const contentBlob = new Blob([JSON.stringify(content)], {
    type: "application/json",
  });
  return (
    <>
      <section className="section">
        <nav className="breadcrumb">
          <ul>
            <li>
              <Link to="/">Top</Link>
            </li>
            <li>
              <Link to={`/contests/${graph.contest_name}/graphs`}>
                {graph.contest_name}
              </Link>
            </li>
            <li className="is-active">
              <a>{graph.graph_name}</a>
            </li>
          </ul>
        </nav>
      </section>
      <section className="section">
        <h3 className="title">Edit Graph</h3>
        <Form method="post" encType="multipart/form-data">
          <div className="field">
            <label className="label">Contest Name</label>
            <div className="control">
              <input
                className="input"
                type="text"
                name="contestName"
                value={graph.contest_name}
                readOnly
              />
            </div>
          </div>
          <div className="field">
            <label className="label">Graph Name</label>
            <div className="control">
              <input
                className="input"
                type="text"
                name="graphName"
                value={graph.graph_name}
                readOnly
              />
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
                Save
              </button>
            </div>
          </div>
        </Form>
      </section>
      <section className="section">
        <h3 className="title">Graph Content</h3>
        <div className="field">
          <label className="label">Content</label>
          <div className="control">
            <textarea
              className="textarea"
              value={JSON.stringify(content)}
              readOnly
            />
          </div>
          <p className="help">
            <a
              href={URL.createObjectURL(contentBlob)}
              download={`${graph.contest_name}-${graph.graph_name}.json`}
            >
              Download
            </a>
          </p>
        </div>
      </section>
    </>
  );
}
