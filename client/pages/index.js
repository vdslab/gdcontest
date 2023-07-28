import { fetchContests } from "@/api";
import { Section } from "@/components/Section";
import { Link, Table } from "@nextui-org/react";
import NextLink from "next/link";

export async function getServerSideProps({}) {
  const contests = await fetchContests();
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
                    <NextLink href={`/contests/${contest.contest_name}`}>
                      {contest.contest_name}
                    </NextLink>
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
