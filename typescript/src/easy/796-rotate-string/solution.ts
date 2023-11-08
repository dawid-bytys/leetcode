/*
 * Time: O(n)
 * Space: O(1)
 */
function rotateString(s: string, goal: string): boolean {
  if (s === goal) {
    return true;
  }

  if (s.length !== goal.length) {
    return false;
  }

  for (let i = 0; i < s.length - 1; ++i) {
    s = s.substring(1) + s.substring(0, 1);

    if (s === goal) {
      return true;
    }
  }

  return false;
}
