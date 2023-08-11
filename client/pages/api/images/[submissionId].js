import { fetchGraphContent, fetchSubmission } from "@/api";
import { createCanvas } from "@napi-rs/canvas";

function toImage(graph, drawing) {
  const r = 3;
  const left = Math.min(...graph.nodes.map(({ id }) => drawing[id][0] - r));
  const right = Math.max(...graph.nodes.map(({ id }) => drawing[id][0] + r));
  const top = Math.min(...graph.nodes.map(({ id }) => drawing[id][1] - r));
  const bottom = Math.max(...graph.nodes.map(({ id }) => drawing[id][1] + r));
  const cx = (right + left) / 2;
  const cy = (bottom + top) / 2;
  const size = 1248;
  const canvas = createCanvas(size, size);
  const ctx = canvas.getContext("2d");
  ctx.fillStyle = "white";
  ctx.fillRect(0, 0, size, size);
  ctx.translate(size / 2 - cx, size / 2 - cy);
  for (const { source, target } of graph.links) {
    ctx.beginPath();
    ctx.moveTo(drawing[source][0], drawing[source][1]);
    ctx.lineTo(drawing[target][0], drawing[target][1]);
    ctx.strokeStyle = "#dbdbdb";
    ctx.lineWidth = 2;
    ctx.stroke();
  }
  for (const { id } of graph.nodes) {
    ctx.beginPath();
    ctx.ellipse(drawing[id][0], drawing[id][1], r, r, 0, 0, Math.PI * 2);
    ctx.fillStyle = "#485fc7";
    ctx.fill();
  }
  return canvas.encode("jpeg");
}

async function fetchDrawing(submissionId) {
  const auth = `${process.env.AUTH_USER}:${process.env.AUTH_PASSWORD}`;
  const response = await fetch(
    `${process.env.NEXT_PUBLIC_API_ORIGIN}/admin/submissions/${submissionId}/content`,
    {
      headers: {
        Authorization: `Basic ${Buffer.from(auth).toString("base64")}`,
      },
    },
  );
  return response.json();
}

export default async function getSubmissionImage(req, res) {
  try {
    const { submissionId } = req.query;
    const submission = await fetchSubmission(submissionId);
    const graph = await fetchGraphContent(
      submission.contest_name,
      submission.graph_name,
    );
    const drawing = await fetchDrawing(submissionId);
    const image = await toImage(graph, drawing);
    res.status(200).setHeader("Content-Type", "image/jpeg").send(image);
  } catch (error) {
    console.error(error);
    res.status(error.status || 500).json({
      code: error.code,
      error: error.message,
    });
  }
}
