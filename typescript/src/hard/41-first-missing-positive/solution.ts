/*
 * Time: O(n)
 * Space: O(1)
 */
function firstMissingPositive(nums: number[]) {
  let left = 0;

  while (left < nums.length) {
    const correctIndex = nums[left] - 1;

    if (nums[left] > 0 && nums[left] <= nums.length && nums[left] !== nums[correctIndex]) {
      [nums[left], nums[correctIndex]] = [nums[correctIndex], nums[left]];
    } else {
      ++left;
    }
  }

  for (let i = 0; i < nums.length; ++i) {
    if (nums[i] !== i + 1) {
      return i + 1;
    }
  }

  return nums.length + 1;
}
