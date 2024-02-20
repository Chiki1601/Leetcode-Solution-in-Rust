class Solution {
 int missingNumber(List<int> nums) {
  int sum = (nums.length + 1) * nums.length ~/ 2;
  for (int num in nums) {
    sum -= num;
  }
  return sum;
}
}
