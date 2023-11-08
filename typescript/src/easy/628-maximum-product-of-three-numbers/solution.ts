/*
 * Time: O(n)
 * Space: O(1)
 */
function maximumProduct(nums: number[]) {
  if (nums.length < 3) {
    return null;
  }

  if (nums.length === 3) {
    return nums[0] * nums[1] * nums[2];
  }

  let largest = Number.MIN_SAFE_INTEGER;
  let secondLargest = Number.MIN_SAFE_INTEGER;
  let thirdLargest = Number.MIN_SAFE_INTEGER;

  let smallest = Number.MAX_SAFE_INTEGER;
  let secondSmallest = Number.MAX_SAFE_INTEGER;

  for (const num of nums) {
    if (num > largest) {
      thirdLargest = secondLargest;
      secondLargest = largest;
      largest = num;
    } else if (num > secondLargest) {
      thirdLargest = secondLargest;
      secondLargest = num;
    } else if (num > thirdLargest) {
      thirdLargest = num;
    }

    if (num < smallest) {
      secondSmallest = smallest;
      smallest = num;
    } else if (num < secondSmallest) {
      secondSmallest = num;
    }
  }

  const positiveProduct = thirdLargest * secondLargest * largest;
  const negativeProduct = smallest * secondSmallest * largest;

  return Math.max(positiveProduct, negativeProduct);
}
