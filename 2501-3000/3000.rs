impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        dimensions.iter().map(|x| (x[0] * x[0] + x[1] * x[1], x[0] * x[1])).max().unwrap().1
    }
}
