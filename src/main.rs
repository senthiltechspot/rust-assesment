#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }

    fn max_depth(root: &Option<Box<TreeNode>>) -> i32 {
        match root {
            None => 0,
            Some(node) => 1 + std::cmp::max(TreeNode::max_depth(&node.left), TreeNode::max_depth(&node.right)),
        }
    }
}

fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

fn first_occurrence(nums: &[i32], target: i32) -> Option<usize> {
    nums.iter().position(|&x| x == target)
}

fn shortest_word(s: &str) -> &str {
    s.split_whitespace().min_by_key(|word| word.len()).unwrap_or("")
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=n / 2 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn median(nums: &[i32]) -> f64 {
    let len = nums.len();
    if len % 2 == 0 {
        let mid = len / 2;
        (nums[mid - 1] + nums[mid]) as f64 / 2.0
    } else {
        nums[len / 2] as f64
    }
}

fn longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    let mut prefix = strings[0].to_string();
    for s in &strings[1..] {
        while !s.starts_with(&prefix) {
            prefix.pop();
        }
    }
    prefix
}

fn kth_smallest(nums: &[i32], k: usize) -> Option<i32> {
    nums.iter().cloned().nth(k - 1)
}

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn merge_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
    let mut merged = Vec::new();
    while !nums1.is_empty() && !nums2.is_empty() {
        if nums1[0] < nums2[0] {
            merged.push(nums1.remove(0));
        } else {
            merged.push(nums2.remove(0));
        }
    }
    merged.extend(nums1);
    merged.extend(nums2);
    merged
}

fn max_subarray_sum(nums: &[i32]) -> i32 {
    let mut max_sum = nums[0];
    let mut current_sum = nums[0];
    for &num in nums.iter().skip(1) {
        current_sum = current_sum.max(num);
        max_sum = max_sum.max(current_sum);
    }
    max_sum
}

fn main() {
    // Test each function

    // 1. Implement a function that checks whether a given string is a palindrome or not.
    let test_string = "radar";
    println!("Is '{}' a palindrome? {}", test_string, is_palindrome(test_string));

    // 2. Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.
    let nums = vec![1, 2, 3, 4, 4, 5, 6];
    let target = 4;
    println!("First occurrence of {}: {:?}", target, first_occurrence(&nums, target));

    // 3. Given a string of words, implement a function that returns the shortest word in the string.
    let test_string = "This is a test string";
    println!("Shortest word: {}", shortest_word(test_string));

    // 4. Implement a function that checks whether a given number is prime or not.
    let test_number = 17;
    println!("Is {} prime? {}", test_number, is_prime(test_number));

    // 5. Given a sorted array of integers, implement a function that returns the median of the array.
    let nums = vec![1, 2, 3, 4, 5];
    println!("Median: {}", median(&nums));

    // 6. Implement a function that finds the longest common prefix of a given set of strings.
    let words = vec!["flower", "flow", "flight"];
    println!("Longest common prefix: {}", longest_common_prefix(&words));

    // 7. Implement a function that returns the kth smallest element in a given array.
    let nums = vec![1, 3, 5, 7, 9];
    let k = 3;
    match kth_smallest(&nums, k) {
        Some(val) => println!("{}{} smallest element: {:?}", k, match k {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th"
        }, val),
        None => println!("No such element found")
    }

    // 8. Given a binary tree, implement a function that returns the maximum depth of the tree.
    let root = Some(Box::new(TreeNode {
        val: 1,
        left: Some(Box::new(TreeNode::new(2))),
        right: Some(Box::new(TreeNode::new(3))),
    }));
    println!("Maximum depth of the tree: {}", TreeNode::max_depth(&root));

    // 9. Reverse a string in Rust
    let test_string = "hello";
    println!("Reversed string: {}", reverse_string(test_string));

    // 10. Check if a number is prime in Rust
    let test_number = 17;
    println!("Is {} prime? {}", test_number, is_prime(test_number));

    // 11. Merge two sorted arrays in Rust
    let nums1 = vec![1, 3, 5];
    let nums2 = vec![2, 4, 6];
    println!("Merged array: {:?}", merge_sorted_arrays(nums1, nums2));

    // 12. Find the maximum subarray sum in Rust
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("Maximum subarray sum: {}", max_subarray_sum(&nums));
}
