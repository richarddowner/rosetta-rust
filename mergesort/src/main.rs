fn mergesort(left: uint, right: uint, list:&[uint]) -> () {
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

fn merge(list:&[uint], l_start: uint, l_end:uint, r_start:uint, r_end:uint) -> () {

	// temp lists for comparison
	let mut l_half: Vec<uint> = Vec::new();
	let mut r_half: Vec<uint> = Vec::new();

	let mut i = l_start;
	let mut l = 0_u;
	let mut r = 0_u;

	// copy values into temp lists
	while i < l_end {
		l_half.insert(l, list[i] as uint);
		i = i + 1; l = l + 1;
	}
	i = r_start;
	while i < r_end {
		r_half.insert(r, list[i] as uint);
		i = i + 1; r = r + 1;
	}

	// merge the values back into positions in the main list
	i = l_start; r = 0_u; l = 0_u;
	while l < l_half.len() && r < r_half.len() {
		// if left value < r value, move left value
		if(l_half[l] < r_half[r]) {
			list[i] = l_half[l];
			l = l + 1;
		} else {
			list[i] = r_half[r];
			r = r + 1;
		}		
		i = i + 1;
	}

	// handle leftover values
	while l < l_half.len() {
		list[i] = l_half[l];
		i = i + 1; l = l + 1;
	}
	while r < r_half.len() {
		list[i] = r_half[r];
		i = i + 1; r = r + 1;
	}
}

fn main() {
	let mut unsorted: [uint, ..10] = [1, 5, 2, 9, 8, 4, 3, 6, 7, 0];
	mergesort(0, unsorted.len(), &unsorted);
}
