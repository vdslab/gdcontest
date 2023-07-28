import { withPageAuthRequired } from "@auth0/nextjs-auth0";
import {
  Button,
  Input,
  Link,
  Spacer,
  Table,
  useInput,
} from "@nextui-org/react";
import { Section } from "@/components/Section";
import { useRouter } from "next/router";
import { fetchContest, fetchGraphs } from "@/api";

export async function getServerSideProps(req) {
  const { contestName } = req.query;
  const contest = await fetchContest(contestName);
  const graphs = await fetchGraphs(contestName);
  return {
    props: {
      contest,
      graphs,
    },
  };
}

export default function ContestDetailPage({ contest, graphs }) {
  return (
    <>
      <Section>{contest.contest_name}</Section>
      <Section>
        <Table aria-label="graphs">
          <Table.Header>
            <Table.Column>Graph Name</Table.Column>
            <Table.Column>Action</Table.Column>
          </Table.Header>
          <Table.Body>
            {graphs.map((graph) => {
              return (
                <Table.Row key={graph.graph_name}>
                  <Table.Cell>{graph.graph_name}</Table.Cell>
                  <Table.Cell>
                    <Link
                      href={`http://localhost:8080/contests/${graph.contest_name}/graphs/${graph.graph_name}/content`}
                      download={`${graph.contest_name}-${graph.graph_name}.json`}
                      target="_blank"
                      rel="noopener noreferrer"
                      isExternal
                    >
                      Download
                    </Link>
                    <br />
                    <Link
                      href={`/contests/${graph.contest_name}/${graph.graph_name}`}
                    >
                      Submit
                    </Link>
                  </Table.Cell>
                </Table.Row>
              );
            })}
          </Table.Body>
        </Table>
      </Section>
    </>
  );
}
