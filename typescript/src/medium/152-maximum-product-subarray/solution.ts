/*
 * Time: O(n)
 * Space: O(1)
 */
function maxProduct(nums: number[]) {
  if (nums.length === 1) {
    return nums[0];
  }

  let maxProduct = Math.max(...nums);
  let currentMax = 1;
  let currentMin = 1;

  for (const num of nums) {
    const temp = currentMax * num;
    currentMax = Math.max(currentMax * num, currentMin * num, num);
    currentMin = Math.min(temp, currentMin * num, num);
    maxProduct = Math.max(maxProduct, currentMax);
  }

  return maxProduct;
}
