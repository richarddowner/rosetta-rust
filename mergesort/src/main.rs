fn main() {
	let mut list = vec![1_u64, 5, 2, 9, 8, 4, 3, 6, 7, 0];
	mergesort(0, list.len() as u64, &mut list);
}

fn mergesort(left: u64, right: u64, list: &mut Vec<u64>) -> () {
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

fn merge(list: &mut Vec<u64>, l_start:u64, l_end:u64, r_start:u64, r_end:u64) -> () {

	// temp lists for comparison
	let mut l_half = Vec::new();
	let mut r_half = Vec::new();

	let mut i = l_start;
	let mut l = 0_u64;
	let mut r = 0_u64;

	// copy values into temp lists
	while i < l_end {
		l_half[l as uint] = list[i as uint];
		i += 1; l += 1;
	}
	i = r_start;
	while i < r_end {
		r_half[r as uint] = list[i as uint];
		i += 1; r += 1;
	}

	// merge the values back into positions in the main list
	i = l_start; r = 0_u64; l = 0_u64;
	while l < l_half.len() as u64 && r < r_half.len() as u64 {
		if(l_half[l as uint] < r_half[r as uint]) {
			list[i as uint] = l_half[l as uint];
			l += 1;
		} else {
			list[i as uint] = r_half[r as uint];
			r += 1;
		}		
		i += 1;
	}

	// handle leftover values
	while l < l_half.len() as u64 {
		list[i as uint] = l_half[l as uint];
		i += 1; l += 1;
	}
	while r < r_half.len() as u64 {
		list[i as uint]= r_half[r as uint];
		i += 1; r += 1;
	}
}
