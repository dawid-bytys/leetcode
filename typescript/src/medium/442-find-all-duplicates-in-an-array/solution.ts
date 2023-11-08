/*
 * Time: O(n)
 * Space: O(1) - tricky, but it is
 */
function findDuplicates(nums: number[]) {
  const result: number[] = [];

  for (let i = 0; i < nums.length; ++i) {
    const idx = Math.abs(nums[i]) - 1;

    if (nums[idx] < 0) {
      result.push(idx + 1);
    } else {
      nums[idx] *= -1;
    }
  }

  return result;
}
