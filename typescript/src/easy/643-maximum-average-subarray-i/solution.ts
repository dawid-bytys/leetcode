/*
 * Time: O(k + n);
 * Space: O(1)
 */
function findMaxAverage(nums: number[], k: number) {
  if (nums.length === 1) {
    return nums[0];
  }

  if (k === 1) {
    return Math.max(...nums);
  }

  let currentSum = nums.slice(0, k).reduce((acc, el) => acc + el, 0);
  let left = 1;
  let right = left + k - 1;
  let currentMaxAvg = currentSum / k;

  while (right < nums.length) {
    currentSum -= nums[left - 1];
    currentSum += nums[right];
    currentMaxAvg = Math.max(currentMaxAvg, currentSum / k);

    ++left;
    ++right;
  }

  return currentMaxAvg;
}
