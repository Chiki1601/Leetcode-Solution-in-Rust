impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut total_beams = 0;
        let mut prev_device_count = 0;
        
        // Iterate through each row of the bank
        for row in bank.iter() {
            let mut current_device_count = 0;
            
            // Count security devices ('1's) in current row
            for cell in row.bytes() {
                current_device_count += (cell & 1) as i32;  // Bit trick: '1' & 1 = 1, '0' & 1 = 0
            }
            
            // If current row has devices, calculate beams with previous row
            if current_device_count > 0 {
                total_beams += prev_device_count * current_device_count;
                prev_device_count = current_device_count;
            }
        }
        
        total_beams
    }
}
