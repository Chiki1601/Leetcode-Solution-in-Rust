impl Solution {
	pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
		let mut ans = 0;
		if nums2.len() % 2 == 1 {
			for &x in &nums1 { ans ^= x; }
		}
		if nums1.len() % 2 == 1 {
			for &x in &nums2 { ans ^= x; }
		}
		ans
	}
}
