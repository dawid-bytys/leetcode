/*
 * Time: O(n * log(n))
 * Space: O(1)
 */
function findMin(nums: number[]) {
  let start = 0;
  let end = nums.length - 1;

  while (start < end) {
    const minIdx = Math.floor((start + end) / 2);

    if (nums[minIdx] > nums[end]) {
      start = minIdx + 1;
    } else {
      end = minIdx;
    }
  }

  return nums[start];
}
