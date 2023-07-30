import { Form, Link, useLoaderData } from "react-router-dom";
import { callApi } from "../callApi";

export async function loader({ params }) {
  const contest = await callApi(`/contests/${params.contestName}`);
  return { contest };
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

export default function ContestDetail() {
  const { contest } = useLoaderData();
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
        <h3 className="title">Edit Contest</h3>
        <Form method="post">
          <div className="field">
            <label className="label">Contest Name</label>
            <div className="control">
              <input
                className="input"
                type="text"
                name="contest_name"
                value={contest.contest_name}
                readOnly
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
                defaultValue={contest.start_at}
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
                defaultValue={contest.end_at}
                required
              />
            </div>
          </div>
          <div className="field">
            <div className="control">
              <label className="checkbox">
                <input
                  type="checkbox"
                  name="published"
                  defaultChecked={contest.published}
                />{" "}
                Publish
              </label>
            </div>
          </div>
          <div className="field">
            <div className="control buttons">
              <button type="submit" className="button is-primary">
                Save
              </button>
              <Link
                to={`/contests/${contest.contest_name}/graphs`}
                className="button"
              >
                Show Graphs
              </Link>
            </div>
          </div>
        </Form>
      </section>
    </>
  );
}
