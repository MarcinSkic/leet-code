use std::collections::HashMap;

pub fn two_sum_memory_efficient(nums: Vec<i32>, target: i32) -> Vec<i32> {
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

pub fn two_sum_time_efficient(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    let mut h: HashMap<i32, usize> = HashMap::new();

    for (index, num) in nums.iter().enumerate() {
        if let Some(x) = h.get(num) {
            v.push(index.try_into().unwrap());
            v.push((*x).try_into().unwrap());

            return v;
        }

        h.insert(target - num, index);
    }

    v
}
