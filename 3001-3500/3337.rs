impl Solution {
    const MOD:u64=1000000007;

    pub fn length_after_transformations(s: String, t: i32, nums: Vec<i32>) -> i32 {
        let mut matrix=[[0u64; 26]; 26];
        for i in 0..26{
            for j in i+1..=i+nums[i] as usize{
                matrix[j%26][i]=1;
            }
        }

        let M=Self::mexp(matrix, t);

        let mut v=[0u64; 26];
        for c in s.as_bytes(){
            v[(*c-b'a') as usize]+=1;
        }
        let mut r=[0u64; 26];
        for i in 0..26{
            let mut n=0;
            for j in 0..26{
                n+=M[i][j]*v[j];
                n%=Self::MOD;
            }
            r[i]=n;
        }
        (r.into_iter().sum::<u64>()%Self::MOD) as i32
    }

    fn mexp(matrix: [[u64; 26]; 26], p: i32) -> [[u64; 26]; 26]{
        if p==1{
            matrix
        }else{
            let h=Self::mexp(matrix, p/2);
            if p%2==0{
                Self::mm(&h, &h)
            }else{
                let r=Self::mm(&h, &h);
                Self::mm(&r, &matrix)
            }
        }
    }

    fn mm(A: &[[u64; 26]; 26], B: &[[u64; 26]; 26]) -> [[u64; 26]; 26]{
        let mut res=[[0u64; 26]; 26];
        for i in 0..26{
            for j in 0..26{
                let mut n=0;
                for k in 0..26{
                    n+=A[i][k]*B[k][j];
                    n%=Self::MOD;
                }
                res[i][j]=n;
            }
        }
        res
    }
}
