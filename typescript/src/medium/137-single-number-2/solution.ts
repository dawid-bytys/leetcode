/*
 * Time: O(n)
 * Space: O(n)
 */
function singleNumber(nums: number[]) {
  const numOccurrences: Map<number, number> = new Map();

  for (const num of nums) {
    numOccurrences.set(num, (numOccurrences.get(num) || 0) + 1);
  }

  for (const [key, value] of numOccurrences.entries()) {
    if (value === 1) {
      return key;
    }
  }

  return -1;
}
