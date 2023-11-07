/*
 * Time: O(n * log(n) + n^2)
 * Space: O(1)
 */
function triangleNumber(nums: number[]) {
  if (nums.length < 3) {
    return 0;
  }

  nums.sort((a, b) => a - b);

  let count = 0;

  for (let i = nums.length - 1; i > 1; --i) {
    let left = 0;
    let right = i - 1;

    while (left < right) {
      const canFormTriangle = nums[left] + nums[right] > nums[i];

      if (canFormTriangle) {
        count += right - left;
        --right;
      } else {
        ++left;
      }
    }
  }

  return count;
}
