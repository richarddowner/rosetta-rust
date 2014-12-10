fn sort<T: Clone + PartialOrd>(list: &[T]) -> Vec<T> {
	if list.len() <= 1 {
		return list.to_vec();	
	}	
	let left = list.slice(0u, (list.len() / 2));
	let right = list.slice(left.len(), list.len());
	return merge(sort(left).as_slice(), sort(right).as_slice());
}
fn merge<T: Clone + PartialOrd>(left: &[T], right: &[T]) -> Vec<T> {
	let mut result = Vec::with_capacity(left.len() + right.len()); 
	let mut left_pos = 0u;
	let mut right_pos = 0u;
	while left_pos < left.len() && right_pos < right.len() {
		if left[left_pos].le(&right[right_pos]) {
			result.push(left[left_pos].clone());
			left_pos += 1; 
		} else {
			result.push(right[right_pos].clone());
			right_pos += 1; 
		}
	}
	while left_pos < left.len() {
		result.push(left[left_pos].clone());
		left_pos += 1;
	}	
	while right_pos < right.len() {
		result.push(right[right_pos].clone());
		right_pos += 1;
	}
	return result;
}
#[test]
fn mergesort_test() {
	let list: [int, ..10] = [2, 3, 1, 8, 6, 5, 9, 4, 10, 7];
	let correct_order: [int, ..10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
	let sorted_list = sort(list);
	assert_eq!(sorted_list, correct_order.to_vec()); 
}
