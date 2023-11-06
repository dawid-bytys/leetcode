/*
 * Time: O(nlogn + n^2)
 * Space: O(1)
 */
function threeSumClosest(nums: number[], target: number) {
  nums.sort((a, b) => a - b);

  let currentClosestSum = Number.MAX_SAFE_INTEGER;
  let currentDifference = Number.MAX_SAFE_INTEGER;

  for (let i = 0; i < nums.length - 2; ++i) {
    let left = i + 1;
    let right = nums.length - 1;

    while (left < right) {
      const currentSum = nums[i] + nums[left] + nums[right];

      if (currentSum === target) {
        return currentSum;
      }

      const diff = Math.abs(target - currentSum);

      if (diff < currentDifference) {
        currentDifference = diff;
        currentClosestSum = currentSum;
      }

      if (currentSum > target) {
        --right;
      } else {
        ++left;
      }
    }
  }

  return currentClosestSum;
}
