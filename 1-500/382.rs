use rand::prelude::*;

struct Solution {
    len: usize,
    head: Option<Box<ListNode>>,
    rng: ThreadRng,
}

impl Solution {

    fn new(head: Option<Box<ListNode>>) -> Self {
        let mut t = &head;
        let mut len = 0;
        let mut rng = thread_rng();

        while let Some(node) = t {
            len += 1;
            t = &node.next;
        }

        Self { len, head, rng }
    }
    
    fn get_random(&mut self) -> i32 {
        let mut i = self.rng.gen_range(0, self.len);
        let mut t = &self.head;

        while i > 0 {
            t = &t.as_ref().unwrap().next;
            i -= 1;
        }
        t.as_ref().unwrap().val
    }
}
