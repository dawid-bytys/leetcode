/*
 * Time: O(n)
 * Space: O(n)
 */
function containsNearbyDuplicate(nums: number[], k: number) {
  const map = new Map<number, number>();

  for (let i = 0; i < nums.length; ++i) {
    if (map.has(nums[i]) && Math.abs(i - (map.get(nums[i]) || 0)) <= k) {
      return true;
    }

    map.set(nums[i], i);
  }

  return false;
}
