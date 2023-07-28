import {
  Button,
  Collapse,
  Input,
  Link,
  Spacer,
  Table,
  Textarea,
} from "@nextui-org/react";
import { Section } from "@/components/Section";
import {
  fetchContest,
  fetchGraph,
  fetchGraphContent,
  fetchStanginsSubmissions,
} from "@/api";
import { useRef } from "react";
import { useUser } from "@auth0/nextjs-auth0/client";
import { useRouter } from "next/router";

export async function getServerSideProps(req) {
  const { contestName, graphName } = req.query;
  const contest = await fetchContest(contestName);
  const graph = await fetchGraph(contestName, graphName);
  const graphContent = await fetchGraphContent(contestName, graphName);
  const submissions = await fetchStanginsSubmissions(contestName, graphName);
  return {
    props: {
      contest,
      graph,
      graphContent,
      submissions,
    },
  };
}

export default function GraphDetailPage({ graph, graphContent, submissions }) {
  const router = useRouter();
  const { user } = useUser();
  const fileRef = useRef();
  const textareaRef = useRef();
  const graphContentBlob = new Blob([JSON.stringify(graphContent)], {
    type: "application/json",
  });
  return (
    <>
      <Section>
        <Collapse.Group>
          <Collapse title="Data" expanded>
            <Input
              label="Contest Name"
              fullWidth
              value={graph.contest_name}
              readOnly
            />
            <Spacer />
            <Input
              label="Graph Name"
              fullWidth
              value={graph.graph_name}
              readOnly
            />
            <Spacer />
            <Input
              label="Number of Nodes"
              fullWidth
              value={graphContent.nodes.length}
              readOnly
            />
            <Spacer />
            <Input
              label="Number of Links"
              fullWidth
              value={graphContent.links.length}
              readOnly
            />
            <Spacer />
            <Link
              href={URL.createObjectURL(graphContentBlob)}
              download={`${graph.contest_name}-${graph.graph_name}.json`}
              isExternal
            >
              Download
            </Link>
          </Collapse>
          <Collapse title="Submit">
            <form
              onSubmit={async (event) => {
                event.preventDefault();
                const request = await fetch(
                  `/api/submissions/${graph.contest_name}/${graph.graph_name}`,
                  {
                    method: "POST",
                    body: JSON.stringify(textareaRef.current.value),
                    headers: {
                      "Content-Type": "application/json",
                    },
                  },
                );
                await request.json();
                router.replace(router.asPath);
              }}
            >
              <input
                ref={fileRef}
                type="file"
                style={{ display: "none" }}
                onChange={async (event) => {
                  textareaRef.current.value =
                    await event.target.files[0].text();
                }}
              />
              <Textarea
                ref={textareaRef}
                label="Your Submission"
                fullWidth
                disabled={!user}
                helperText={user ? null : "Login to Submit"}
                required
              />
              <Spacer />
              <Button.Group bordered disabled={!user}>
                <Button
                  onClick={() => {
                    fileRef.current.click();
                  }}
                >
                  Upload from File
                </Button>
                <Button type="submit">Submit</Button>
              </Button.Group>
            </form>
          </Collapse>
          <Collapse title="Standings">
            <Table aria-label="Standings">
              <Table.Header>
                <Table.Column>Position</Table.Column>
                <Table.Column>User</Table.Column>
                <Table.Column>Score</Table.Column>
              </Table.Header>
              <Table.Body>
                {submissions.map((submission, i) => {
                  return (
                    <Table.Row key={i}>
                      <Table.Cell>{i + 1}</Table.Cell>
                      <Table.Cell>{submission.user_id}</Table.Cell>
                      <Table.Cell>{submission.score}</Table.Cell>
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
