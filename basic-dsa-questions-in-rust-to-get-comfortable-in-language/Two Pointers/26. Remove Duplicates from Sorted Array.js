var removeDuplicates = function (nums) {
  let left = 1;
  for (let i = 0; i < nums.length - 1; i++) {
    if (nums[i] !== nums[i + 1]) {
      nums[left] = nums[i + 1];
      left++;
    }
  }
  return left;
};
