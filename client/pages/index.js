import { fetchContests } from "@/api";
import Link from "next/link";

export async function getStaticProps({}) {
  const contests = await fetchContests();
  return {
    props: {
      contests,
    },
    revalidate: 10,
  };
}

export default function HomePage({ contests }) {
  return (
    <>
      <div className="block">
        <h3 className="title">Contests</h3>
        <table className="table is-bordered is-fullwidth">
          <thead>
            <tr>
              <th>Contest Name</th>
              <th>Start</th>
              <th>End</th>
            </tr>
          </thead>
          <tbody>
            {contests.map((contest) => {
              return (
                <tr key={contest.contest_name}>
                  <td>
                    <Link href={`/contests/${contest.contest_name}`}>
                      {contest.contest_name}
                    </Link>
                  </td>
                  <td>{contest.start_at}</td>
                  <td>{contest.end_at}</td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
    </>
  );
}
