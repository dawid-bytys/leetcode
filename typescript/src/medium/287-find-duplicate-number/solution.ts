/*
 * Time: O(n)
 * Space: O(n)
 */
function findDuplicate(nums: number[]) {
  const set = new Set();

  for (const num of nums) {
    if (set.has(num)) {
      return num;
    }

    set.add(num);
  }

  return null;
}
