use std::cmp;

pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    // Sort two arrays.
    let mut nums1 = nums1.clone();
    nums1.sort();
    let mut nums2 = nums2.clone();
    nums2.sort();
    // Get the iterators.
    let mut iter1 = nums1.iter();
    let mut iter2 = nums2.iter();
    // Init value.
    let mut num1 = iter1.next();
    let mut num2 = iter2.next();
    // Result vector.
    let mut res: Vec<i32> = vec![];
    loop {
        if num1 == None || num2 == None {
            break;
        }
        match num1.unwrap().cmp(num2.unwrap()) {
            cmp::Ordering::Less => num1 = iter1.next(),
            cmp::Ordering::Greater => num2 = iter2.next(),
            cmp::Ordering::Equal => {
                res.push(*num1.unwrap());
                num1 = iter1.next();
                num2 = iter2.next();
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        assert_eq!(intersect(nums1, nums2), vec![2, 2]);
    }

    #[test]
    fn test_2() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        assert_eq!(intersect(nums1, nums2), vec![4, 9]);
    }
}
