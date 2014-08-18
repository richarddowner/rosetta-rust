// A happy number is defined by the following process.
// Starting with any positive int, replace the number
// by the sum of the squares of its digits, and repeat
// until the number equals 1, or it loops endlessly in
// a cycle which does not include 1.
// Those numbers for which this process ends in 1 are
// happy numbers.

fn happy_nums(mut input: uint) -> bool {
	let mut tenths = 0u;
	let mut n = 0u;

	let mut limit = 1000u;

	if input == 1 {
		return true;
	}

	while input != 1 && limit != 0 {
		n = 0;
		while input > 0 {
			tenths = input % 10;
			n += tenths * tenths;
			input /= 10;
		}
		input = n;
		limit = limit - 1;
	}
	input == 1
}

#[allow(dead_code)]
fn main(){
	let mut count = 0u;
	let mut i = 1u;
	loop {
		if count == 8 {
			break;
		}
		if happy_nums(i) {
			println!("{}", i);
			count += 1;
		}
		i += 1;
	}
}

#[test]
fn first_eight_happy_nums_test() {
	assert_eq!(happy_nums(1), true);
	assert_eq!(happy_nums(7), true);
	assert_eq!(happy_nums(10), true);
	assert_eq!(happy_nums(13), true);
	assert_eq!(happy_nums(19), true);
	assert_eq!(happy_nums(23), true);
	assert_eq!(happy_nums(28), true);
	assert_eq!(happy_nums(31), true);
}

#[test]
#[should_fail]
fn unhappy_nums_test() {
	assert_eq!(happy_nums(2), true);
	assert_eq!(happy_nums(11), true);
	assert_eq!(happy_nums(15), true);
}
