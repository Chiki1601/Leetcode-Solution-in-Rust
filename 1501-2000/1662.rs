impl Solution 
{
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool 
    {
        // flatten two lists...
        let it1 = word1.iter().flat_map(|s| s.chars());
        let it2 = word2.iter().flat_map(|s| s.chars());
		
		// ...and compare the resulting itetrators   
        it1.eq(it2)
    }
}
