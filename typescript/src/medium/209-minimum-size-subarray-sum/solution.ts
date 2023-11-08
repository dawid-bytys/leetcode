/*
 * Time: O(n)
 * Space: O(1)
 */
function minSubArrayLen(target: number, nums: number[]) {
  if (nums.length === 0) {
    return 0;
  }

  let currentMinimalLength = Number.MAX_SAFE_INTEGER;
  let currentSum = 0;
  let left = 0;
  let right = 0;

  while (right < nums.length) {
    currentSum += nums[right];

    while (currentSum >= target) {
      currentMinimalLength = Math.min(currentMinimalLength, right - left + 1);
      currentSum -= nums[left];
      ++left;
    }

    ++right;
  }

  return currentMinimalLength === Number.MAX_SAFE_INTEGER ? 0 : currentMinimalLength;
}
