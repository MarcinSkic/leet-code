pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();

    for (first_index, first_num) in nums.iter().enumerate() {
        for (second_index, second_num) in nums[first_index + 1..nums.len()].iter().enumerate() {
            if first_num + second_num == target {
                v.push(first_index.try_into().unwrap());
                v.push((second_index + first_index + 1).try_into().unwrap());

                return v;
            }
        }
    }

    v
}
