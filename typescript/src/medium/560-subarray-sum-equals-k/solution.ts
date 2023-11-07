/*
 * Time: O(n)
 * Space: O(n)
 */
function subarraySum(nums: number[], k: number) {
  const map: Map<number, number> = new Map();
  let currentSum = 0;
  let count = 0;

  for (let i = 0; i < nums.length; ++i) {
    currentSum += nums[i];

    if (currentSum === k) {
      ++count;
    }

    if (map.has(currentSum - k)) {
      count += map.get(currentSum - k) || 0;
    }

    map.set(currentSum, (map.get(currentSum) || 0) + 1);
  }

  return count;
}
