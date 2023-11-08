/*
 * Time: O(n^3)
 * Space: O(1)
 */
function largestTriangleArea(points: number[][]): number {
  let currentMaxArea = Number.MIN_VALUE;

  for (let i = 0; i < points.length - 2; ++i) {
    for (let j = i + 1; j < points.length - 1; ++j) {
      for (let k = j + 1; k < points.length; ++k) {
        const x1 = points[i][0];
        const y1 = points[i][1];
        const x2 = points[j][0];
        const y2 = points[j][1];
        const x3 = points[k][0];
        const y3 = points[k][1];

        const currentArea = Math.abs(x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)) / 2;
        currentMaxArea = Math.max(currentMaxArea, currentArea);
      }
    }
  }

  return currentMaxArea;
}
