fn main() {
    let solution = Solution::search([1,2,3,4,5].to_vec(), 5);
    println!("{}", solution)
}

struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0;
        let mut high = nums.len();
        let mut mid = low + ((high - low) / 2);
        let mut current = nums[mid];
        while low <= high { match current.cmp(&target) {
            std::cmp::Ordering::Equal => return mid as i32,
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid - 1,
            }
            mid = low + ((high - low) / 2);
            current = nums[mid]
        }
        
        return -1;
    }
}
