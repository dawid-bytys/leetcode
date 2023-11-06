/*
 * Time: O(nlogn + n)
 * Space: O(1)
 */
function maximumGap(nums: number[]) {
  if (nums.length === 1) {
    return 0;
  }

  nums.sort((a, b) => a - b);

  let currentMaxDiff = Number.MIN_SAFE_INTEGER;

  for (let i = 0; i < nums.length - 1; ++i) {
    const currentDiff = nums[i + 1] - nums[i];
    currentMaxDiff = Math.max(currentDiff, currentMaxDiff);
  }

  return currentMaxDiff;
}
