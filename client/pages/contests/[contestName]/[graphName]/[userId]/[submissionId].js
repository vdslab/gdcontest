import Image from "next/image";
import Link from "next/link";
import { useRouter } from "next/router";

export default function UserSubmissionDetailPage() {
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
            <li>
              <Link
                href={`/contests/${router.query.contestName}/${router.query.graphName}/${router.query.userId}`}
              >
                {router.query.userId}
              </Link>
            </li>
            <li className="is-active">
              <a>{router.query.submissionId}</a>
            </li>
          </ul>
        </nav>
      </div>
      <div className="block">
        <h3 className="title">Drawing Result</h3>
        <figure className="image is-square">
          {router.query.submissionId && (
            <Image
              src={`/api/images/${router.query.submissionId}`}
              alt={`Drawing result of submission ${router.query.submissionId}`}
              width="1248"
              height="1248"
            />
          )}
        </figure>
      </div>
    </>
  );
}
