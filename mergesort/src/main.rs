fn mergesort(left: uint, right: uint, list:&[int]) -> () {
	// base case
	if(right - left <= 1) { 
		return; 
	}

	// get array indices
	let l_start = left;
	let l_end = (left+right)/2;
	let r_start = l_end;
	let r_end = right;

	// recursive call on left half
	mergesort(l_start, l_end, list);
	// recursive call on right half
	mergesort(r_start, r_end, list);
	// merge sorted right and left halves back together
	merge(list, l_start, l_end, r_start, r_end);	
}

fn merge(list:&[int], l_start: uint, l_end:uint, r_start:uint, r_end:uint) -> () {
	// temp list sizes
	let l_len = l_end - l_start;
	let r_len = r_end - r_start;

	// temp lists for comparison
	let mut l_half: [uint, ..l_len];
	let mut r_half: [uint, ..r_len];	
}

//#[test]
fn main() {
	let mut unsorted: [int, ..10] = [1, 5, 2, 9, 8, 4, 3, 6, 7, 0];
	mergesort(0, unsorted.len(), &unsorted);
}
