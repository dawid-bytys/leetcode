/*
 * Time: O(n)
 * Space: O(n)
 */
function lengthOfLongestSubstring(s: string) {
  if (s.length === 1) {
    return 1;
  }

  const set = new Set();
  let start = 0;
  let left = 0;
  let right = s.length - 1;
  let currentLongest = 0;

  while (left <= right) {
    const leftChar = s.charAt(left);

    if (!set.has(leftChar)) {
      set.add(leftChar);
      ++left;
      currentLongest = Math.max(currentLongest, left - start);
    } else {
      set.delete(s.charAt(start));
      ++start;
    }
  }

  return currentLongest;
}
