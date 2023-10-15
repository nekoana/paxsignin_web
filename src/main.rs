    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();

        let chunks = nums.chunks(3);

        for chunk in chunks {
            if chunk.len() == 3 {
                if chunk[0] == chunk[1]  && chunk[1] == chunk[2] {
                    continue;
                }

                if chunk[0] == chunk[1] {
                    return chunk[2];
                }

                if chunk[1] == chunk[2] {
                    return chunk[0]
                }
            } else {
                return chunk[0];
            }
        }

        todo!()
    }