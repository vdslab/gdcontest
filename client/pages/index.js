import { Section } from "@/components/Section";
import { Button, Link, Table } from "@nextui-org/react";

export async function getServerSideProps({ req }) {
  console.log(req);
  const response = await fetch(
    `${process.env.API_ORIGIN}/contests/${req.query.contestName}`,
  );
  const contest = await response.json();
  return {
    props: {
      contests,
    },
  };
}

export default function HomePage({ contests }) {
  return (
    <>
      <Section>
        <Table>
          <Table.Header>
            <Table.Column>Contest Name</Table.Column>
            <Table.Column>Start</Table.Column>
            <Table.Column>End</Table.Column>
          </Table.Header>
          <Table.Body>
            {contests.map((contest) => {
              return (
                <Table.Row key={contest.contest_name}>
                  <Table.Cell>
                    <Link href={`/contests/${contest.contest_name}`}>
                      {contest.contest_name}
                    </Link>
                  </Table.Cell>
                  <Table.Cell>{contest.start_at}</Table.Cell>
                  <Table.Cell>{contest.end_at}</Table.Cell>
                </Table.Row>
              );
            })}
          </Table.Body>
        </Table>
      </Section>
    </>
  );
}
