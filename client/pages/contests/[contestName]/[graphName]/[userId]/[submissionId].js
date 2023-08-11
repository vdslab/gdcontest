import { fetchSubmission } from "@/api";
import Image from "next/image";
import Link from "next/link";

export async function getStaticProps({ params }) {
  const { submissionId } = params;
  const submission = await fetchSubmission(submissionId);
  return {
    props: {
      submission,
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

export default function UserSubmissionDetailPage({ submission }) {
  return (
    <>
      <div className="block">
        <nav className="breadcrumb" aria-label="breadcrumbs">
          <ul>
            <li>
              <Link href={`/contests/${submission.contest_name}`}>
                {submission.contest_name}
              </Link>
            </li>
            <li>
              <Link
                href={`/contests/${submission.contest_name}/${submission.graph_name}`}
              >
                {submission.graph_name}
              </Link>
            </li>
            <li>
              <Link
                href={`/contests/${submission.contest_name}/${submission.graph_name}/${submission.user_id}`}
              >
                {submission.user_name || submission.user_id}
              </Link>
            </li>
            <li className="is-active">
              <a>{submission.id}</a>
            </li>
          </ul>
        </nav>
      </div>
      <div className="block">
        <h3 className="title">Drawing Result</h3>
        <figure className="image is-square">
          <Image
            src={`/api/images/${submission.id}`}
            alt={`Drawing result of submission ${submission.id}`}
            width="1248"
            height="1248"
          />
        </figure>
      </div>
    </>
  );
}
