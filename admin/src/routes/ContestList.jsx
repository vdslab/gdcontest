import { Form, Link, useLoaderData } from "react-router-dom";
import { callApi } from "../callApi";

export async function loader() {
  const contests = await callApi("/contests");
  return { contests };
}

export async function action({ request }) {
  const formData = await request.formData();
  const contestName = formData.get("contest_name");
  const contest = {
    published: formData.has("published"),
    start_at: `${formData.get("start_at")}:00`,
    end_at: `${formData.get("end_at")}:00`,
  };
  return callApi(`/contests/${contestName}`, {
    method: "PUT",
    body: JSON.stringify(contest),
    headers: {
      "Content-Type": "application/json",
    },
  });
}

export default function ContestList() {
  const { contests } = useLoaderData();
  return (
    <>
      <section className="section">
        <nav className="breadcrumb">
          <ul>
            <li className="is-active">
              <a>Top</a>
            </li>
          </ul>
        </nav>
      </section>
      <section className="section">
        <h3 className="title">New Contest</h3>
        <Form method="post">
          <div className="field">
            <label className="label">Contest Name</label>
            <div className="control">
              <input
                className="input"
                type="text"
                name="contest_name"
                required
              />
            </div>
          </div>
          <div className="field">
            <label className="label">Start at</label>
            <div className="control">
              <input
                className="input"
                type="datetime-local"
                name="start_at"
                required
              />
            </div>
          </div>
          <div className="field">
            <label className="label">End at</label>
            <div className="control">
              <input
                className="input"
                type="datetime-local"
                name="end_at"
                required
              />
            </div>
          </div>
          <div className="field">
            <div className="control">
              <label className="checkbox">
                <input type="checkbox" name="published" /> Publish
              </label>
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
        <h3 className="title">Contest List</h3>
        <table className="table is-bordered is-fullwidth">
          <thead>
            <tr>
              <th>Contest Name</th>
              <th>Start at</th>
              <th>End at</th>
              <th>Action</th>
            </tr>
          </thead>
          <tbody>
            {contests.map((contest) => {
              return (
                <tr key={contest.contest_name}>
                  <td>{contest.contest_name}</td>
                  <td>{contest.start_at}</td>
                  <td>{contest.end_at}</td>
                  <td>
                    <div className="buttons">
                      <Link
                        to={`/contests/${contest.contest_name}/detail`}
                        className="button is-small"
                      >
                        Edit
                      </Link>
                      <Link
                        to={`/contests/${contest.contest_name}/graphs`}
                        className="button is-small"
                      >
                        Graphs
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
