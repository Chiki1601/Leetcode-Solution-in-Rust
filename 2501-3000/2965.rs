impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len(); //return type is usize, not i32;
        let mut arr = vec![0; n*n];

        let mut a = 0;  //We explicitly need to assign some value, or use options.
        let mut b = 0;

        for i in 0..n{        //i and j's types are determined by n's type. which is usize
            for j in 0..n{
                let val = grid[i][j] as usize - 1; //as usize because, index of an array should not be i32
                // Why? usize - unsigned integer type.
                //1) Safety: i32 might lead to negative numbers -> index out of bounds.
                //2) Performance: Native memory address size is usize.
                //3) Consistency: functions like .len() returns usize. 
                arr[val]+=1;
                if arr[val]==2 {
                    a = grid[i][j];
                }
            }
        }

        for i in 0..n*n{
            if arr[i]==0{
                b=(i+1) as i32;
                break;
            }
        }

        vec![a,b]  //Implicit return without semicolon.
    }
}
