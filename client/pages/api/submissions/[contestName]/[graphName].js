import { withApiAuthRequired, getAccessToken } from "@auth0/nextjs-auth0";

export default withApiAuthRequired(async function (req, res) {
  try {
    if (req.method !== "POST") {
      throw {
        status: 405,
        code: 405,
        message: "Only POST requests allowed",
      };
    }
    const { accessToken } = await getAccessToken(req, res, {});
    const { contestName, graphName } = req.query;
    const response = await fetch(
      `${process.env.NEXT_PUBLIC_API_ORIGIN}/contests/${contestName}/graphs/${graphName}/submissions`,
      {
        method: "POST",
        body: JSON.stringify(req.body),
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${accessToken}`,
        },
      },
    );
    const data = await response.text();
    res.status(response.status || 200).json(data);
  } catch (error) {
    console.error(error);
    res.status(error.status || 500).json({
      code: error.code,
      error: error.message,
    });
  }
});
