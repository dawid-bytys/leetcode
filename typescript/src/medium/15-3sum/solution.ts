/*
 * Time: O(n * log(n) + n^2)
 * Space: O(n)
 */
function threeSum(nums: number[]) {
  if (nums.length < 3) {
    return [];
  }

  nums.sort((a, b) => a - b);

  const triplets: number[][] = [];

  for (let i = 0; i < nums.length - 2; ++i) {
    if (i > 0 && nums[i] === nums[i - 1]) {
      continue;
    }

    let left = i + 1;
    let right = nums.length - 1;

    while (right > left) {
      const currentSum = nums[i] + nums[left] + nums[right];

      if (currentSum === 0) {
        triplets.push([nums[i], nums[left], nums[right]]);

        while (right > left && nums[left] === nums[left + 1]) {
          ++left;
        }

        while (right > left && nums[right] === nums[right - 1]) {
          --right;
        }

        ++left;
        --right;
      } else if (currentSum > 0) {
        --right;
      } else {
        ++left;
      }
    }
  }

  return triplets;
}
