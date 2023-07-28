import {
  fetchContest,
  fetchGraph,
  fetchGraphContent,
  fetchStanginsSubmissions,
} from "@/api";
import { useRef } from "react";
import { useUser } from "@auth0/nextjs-auth0/client";
import Link from "next/link";

export async function getStaticProps({ params }) {
  const { contestName, graphName } = params;
  const contest = await fetchContest(contestName);
  const graph = await fetchGraph(contestName, graphName);
  const graphContent = await fetchGraphContent(contestName, graphName);
  const submissions = await fetchStanginsSubmissions(contestName, graphName);
  return {
    props: {
      contest,
      graph,
      graphContent,
      submissions,
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

export default function GraphDetailPage({ graph, graphContent, submissions }) {
  const { user } = useUser();
  const fileRef = useRef();
  const textareaRef = useRef();
  const graphContentBlob = new Blob([JSON.stringify(graphContent)], {
    type: "application/json",
  });
  return (
    <>
      <div className="block">
        <nav className="breadcrumb" aria-label="breadcrumbs">
          <ul>
            <li>
              <Link href={`/contests/${graph.contest_name}`}>
                {graph.contest_name}
              </Link>
            </li>
            <li className="is-active">
              <a>{graph.graph_name}</a>
            </li>
          </ul>
        </nav>
      </div>
      <div className="block">
        <h3 className="title">Graph Detail</h3>
        <div className="field">
          <label className="label">Number of Nodes</label>
          <div className="control">
            <input
              className="input"
              value={graphContent.nodes.length}
              readOnly
            />
          </div>
        </div>
        <div className="field">
          <label className="label">Number of Links</label>
          <div className="control">
            <input
              className="input"
              value={graphContent.links.length}
              readOnly
            />
          </div>
        </div>
        <div className="field">
          <div className="control">
            <a
              download={`${graph.contest_name}-${graph.graph_name}.json`}
              className="button"
              onClick={(event) => {
                event.target.href = URL.createObjectURL(graphContentBlob);
              }}
            >
              Download
            </a>
          </div>
        </div>
      </div>
      <div className="block">
        <h3 className="title">Submission</h3>
        <form
          onSubmit={async (event) => {
            event.preventDefault();
            const request = await fetch(
              `/api/submissions/${graph.contest_name}/${graph.graph_name}`,
              {
                method: "POST",
                body: JSON.stringify(textareaRef.current.value),
                headers: {
                  "Content-Type": "application/json",
                },
              },
            );
            await request.json();
          }}
        >
          <input
            ref={fileRef}
            className="is-invisible"
            type="file"
            onChange={async (event) => {
              textareaRef.current.value = await event.target.files[0].text();
            }}
          />
          <div className="field">
            <label className="label">Your Submission</label>
            <div className="control">
              <textarea
                ref={textareaRef}
                className="textarea"
                disabled={!user}
              />
            </div>
          </div>
          <div className="field">
            <div className="control">
              <div className="buttons">
                <button
                  className="button"
                  onClick={(event) => {
                    event.preventDefault();
                    fileRef.current.click();
                  }}
                  disabled={!user}
                >
                  Choose from File
                </button>
                <button
                  className="button is-primary"
                  type="submit"
                  disabled={!user}
                >
                  Submit
                </button>
              </div>
            </div>
          </div>
        </form>
      </div>
      <div className="block">
        <h3 className="title">Standings</h3>
        <table className="table is-bordered is-fullwidth">
          <thead>
            <tr>
              <th>Position</th>
              <th>User</th>
              <th>Score</th>
            </tr>
          </thead>
          <tbody>
            {submissions.map((submission, i) => {
              return (
                <tr key={i}>
                  <td>{i + 1}</td>
                  <td>{submission.user_id}</td>
                  <td>{submission.score}</td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
    </>
  );
}
