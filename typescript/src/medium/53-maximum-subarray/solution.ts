/*
 * Time Complexity: O(n)
 * Space Complexity: O(1)
 */
function maxSubArray(nums: number[]) {
  let maxSum = nums[0];
  let currentSum = maxSum;

  for (let i = 1; i < nums.length; ++i) {
    currentSum = Math.max(nums[i], currentSum + nums[i]);
    maxSum = Math.max(currentSum, maxSum);
  }

  return maxSum;
}
