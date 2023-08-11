export function calculatePoint(standings) {
  const n = standings.length;
  const m = Math.floor(n / 2);
  const median =
    n % 2 == 0
      ? (standings[m - 1].score + standings[m].score) / 2
      : standings[m].score;
  return standings.map(({ score }, i) => {
    if (i == 0) {
      return 5;
    }
    if (i == n - 1) {
      return 1;
    }
    return score <= median ? 3 : 2;
  });
}
