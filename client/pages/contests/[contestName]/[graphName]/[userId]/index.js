import { fetchUserSubmissions } from "@/api";
import Link from "next/link";
import { useRouter } from "next/router";

export async function getStaticProps({ params }) {
  const { contestName, graphName, userId } = params;
  const submissions = await fetchUserSubmissions(
    contestName,
    graphName,
    userId,
  );
  return {
    props: {
      submissions,
      userName: submissions.length ? submissions[0].user_name : userId,
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

export default function UserSubmissionListPage({ submissions, userName }) {
  const router = useRouter();
  return (
    <>
      <div className="block">
        <nav className="breadcrumb" aria-label="breadcrumbs">
          <ul>
            <li>
              <Link href={`/contests/${router.query.contestName}`}>
                {router.query.contestName}
              </Link>
            </li>
            <li>
              <Link
                href={`/contests/${router.query.contestName}/${router.query.graphName}`}
              >
                {router.query.graphName}
              </Link>
            </li>
            <li className="is-active">
              <a>{userName}</a>
            </li>
          </ul>
        </nav>
      </div>
      <div className="block">
        <h3 className="title">Submissions</h3>
        <div className="table-container">
          <table className="table is-bordered is-fullwidth">
            <thead>
              <tr>
                <th className="has-text-centered" style={{ width: "50%" }}>
                  Submission Date
                </th>
                <th className="has-text-centered" style={{ width: "50%" }}>
                  Score
                </th>
              </tr>
            </thead>
            <tbody>
              {submissions.map((submission, i) => {
                return (
                  <tr key={submission.id}>
                    <td className="has-text-left">
                      <Link
                        href={`/contests/${router.query.contestName}/${
                          router.query.graphName
                        }/${encodeURI(router.query.userId)}/${submission.id}`}
                      >
                        {submission.created_at}
                      </Link>
                    </td>
                    <td className="has-text-right">
                      {submission.score.toFixed(3)}
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
