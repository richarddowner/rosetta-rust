fn main() {
	let mut list: Vec<u64> = vec![1_u64, 5, 2, 7, 3, 9, 4, 6];
	println!("unsorted list: {}", list);
	let sortedlist = mergesort(&mut list);
	println!("sorted list:   {}", sortedlist);
}

fn mergesort<T>(list: &mut Vec<T>) -> &mut Vec<T> {
	if list.len() <= 1 { 
		return list;
	}

	return list;
}

