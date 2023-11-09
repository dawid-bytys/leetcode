/*
 * Time: O(2n)
 * Space: O(1) - not counting result array
 */
function findDisappearedNumbers(nums: number[]) {
  const result: number[] = [];

  for (let i = 0; i < nums.length; ++i) {
    const idx = Math.abs(nums[i]) - 1;

    if (nums[idx] > 0) {
      nums[idx] *= -1;
    }
  }

  for (let i = 0; i < nums.length; ++i) {
    if (nums[i] > 0) {
      result.push(i + 1);
    }
  }

  return result;
}
