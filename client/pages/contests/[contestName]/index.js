import NextLink from "next/link";
import { Collapse, Input, Spacer, Table } from "@nextui-org/react";
import { Section } from "@/components/Section";
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
      <Section>
        <Collapse.Group>
          <Collapse title="Contest Detail">
            <Input
              label="Contest Name"
              fullWidth
              value={contest.contest_name}
              readOnly
            />
            <Spacer />
            <Input
              label="Start"
              type="datetime-local"
              fullWidth
              value={contest.start_at}
              readOnly
            />
            <Spacer />
            <Input
              label="End"
              type="datetime-local"
              fullWidth
              value={contest.end_at}
              readOnly
            />
          </Collapse>
          <Collapse title="Graphs" expanded>
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
                        <NextLink
                          href={`/contests/${graph.contest_name}/${graph.graph_name}`}
                        >
                          Show
                        </NextLink>
                      </Table.Cell>
                    </Table.Row>
                  );
                })}
              </Table.Body>
            </Table>
          </Collapse>
        </Collapse.Group>
      </Section>
    </>
  );
}
