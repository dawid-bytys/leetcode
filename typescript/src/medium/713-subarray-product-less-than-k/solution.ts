/*
 * Time: O(n)
 * Space: O(1)
 */
function numSubarrayProductLessThanK(nums: number[], k: number) {
  if (k <= 1) {
    return 0;
  }

  let currentProduct = 1;
  let count = 0;
  let left = 0;
  let right = 0;

  while (right < nums.length) {
    currentProduct *= nums[right];

    while (currentProduct >= k) {
      currentProduct /= nums[left];
      ++left;
    }

    count += right - left + 1;
    ++right;
  }

  return count;
}
