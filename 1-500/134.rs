impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        match gas
            .into_iter()
            .zip(cost.into_iter())
            .map(|(g, c)| g - c)
            .enumerate()
            .fold((0, (0, 0)), |(s, pass @ (_, vm)), (i, v)| match s + v {
                s if s < vm => (s, (i as i32 + 1, s)),
                s => (s, pass),
            }) {
            (s, _) if s < 0 => -1,
            (_, (im, _)) => im,
        }
    }
}
