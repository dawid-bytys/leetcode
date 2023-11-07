/*
 * Time: O(n)
 * Space: O(1)
 */
function maxProduct(nums: number[]) {
  if (nums.length < 2) {
    return null;
  }

  let largest = Number.MIN_VALUE;
  let secondLargest = Number.MIN_VALUE;

  for (const num of nums) {
    if (num > largest) {
      secondLargest = largest;
      largest = num;
    } else if (num > secondLargest) {
      secondLargest = num;
    }
  }

  return (largest - 1) * (secondLargest - 1);
}
