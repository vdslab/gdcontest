import {
  fetchContest,
  fetchGraph,
  fetchGraphContent,
  fetchStanginsSubmissions,
  postSubmission,
} from "@/api";
import { useRef } from "react";
import { useUser } from "@auth0/nextjs-auth0/client";
import Link from "next/link";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useRouter } from "next/router";
import { calculatePoint } from "@/standings";

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
    revalidate: 60,
  };
}

export async function getStaticPaths() {
  return {
    paths: [],
    fallback: "blocking",
  };
}

export default function GraphDetailPage({
  graph,
  graphContent,
  submissions: initialSubmissions,
}) {
  const fileRef = useRef();
  const textareaRef = useRef();
  const router = useRouter();
  const { user, isLoading } = useUser();
  const queryClient = useQueryClient();
  const { data: submissions } = useQuery({
    queryKey: ["submissions"],
    queryFn: () => {
      return fetchStanginsSubmissions(graph.contest_name, graph.graph_name);
    },
    initialData: initialSubmissions,
  });
  const mutation = useMutation({
    mutationFn: (content) => {
      return postSubmission(graph.contest_name, graph.graph_name, content);
    },
    onSuccess: () => {
      textareaRef.current.value = "";
      queryClient.invalidateQueries({ queryKey: ["submissions"] });
      alert("Submission is completed");
    },
  });
  const graphContentBlob = new Blob([JSON.stringify(graphContent)], {
    type: "application/json",
  });
  const points = calculatePoint(submissions);
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
          onSubmit={(event) => {
            event.preventDefault();
            const content = validate(
              graphContent,
              JSON.parse(textareaRef.current.value),
            );
            if (content) {
              mutation.mutate(content);
            } else {
              alert("Invalid submission");
            }
          }}
        >
          <input
            ref={fileRef}
            className="is-invisible"
            type="file"
            onChange={async (event) => {
              textareaRef.current.value = await event.target.files[0].text();
              fileRef.current.value = "";
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
            {!isLoading && !user && (
              <p className="help">
                <Link href={`/api/auth/login?returnTo=${router.asPath}`}>
                  Login
                </Link>{" "}
                required.
              </p>
            )}
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
        <div className="table-container">
          <table className="table is-bordered is-fullwidth">
            <thead>
              <tr>
                <th style={{ width: "25%" }}>Position</th>
                <th style={{ width: "25%" }}>User</th>
                <th style={{ width: "25%" }}>Score</th>
                <th style={{ width: "25%" }}>Point</th>
              </tr>
            </thead>
            <tbody>
              {submissions.map((submission, i) => {
                return (
                  <tr
                    key={i}
                    className={
                      user && submission.user_id === user.sub
                        ? "is-selected"
                        : ""
                    }
                  >
                    <td>{i + 1}</td>
                    <td>
                      {submission.user_nickname ||
                        submission.user_name ||
                        submission.user_id}
                    </td>
                    <td>{submission.score}</td>
                    <td>{points[i]}</td>
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

function validate(graph, drawing) {
  const validatedDrawing = {};
  for (const key of Object.keys(drawing)) {
    validatedDrawing[`${key}`] = [+drawing[key][0], +drawing[key][1]];
  }
  if (Object.keys(validatedDrawing).length !== graph.nodes.length) {
    return null;
  }
  for (const node of graph.nodes) {
    if (
      !validatedDrawing[node.id] ||
      !Number.isFinite(validatedDrawing[node.id][0]) ||
      !Number.isFinite(validatedDrawing[node.id][1])
    ) {
      return null;
    }
  }
  return validatedDrawing;
}
