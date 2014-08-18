// A happy number is defined by the following process.
// Starting with any positive int, replace the number
// by the sum of the squares of its digits, and repeat
// until the number equals 1, or it loops endlessly in
// a cycle which does not include 1.
// Those numbers for which this process ends in 1 are
// happy numbers.

fn happy_nums(mut input: uint) -> uint {
	let mut tenths = 0u;
	let mut n = 0u64;

	while input > 0 {
		tenths = input % 10;
		n += (tenths*tenths) as u64;
		input /= 10;

		println!("{}", n);
		println!("{}", input);
	}

	tenths
}

fn main(){
	println!("{}", happy_nums(19));
}

#[test]
fn first_eight_happy_nums_test() {
	assert_eq!(happy_nums(1), 1);
	assert_eq!(happy_nums(7), 1);
	assert_eq!(happy_nums(10), 1);
	assert_eq!(happy_nums(13), 1);
	assert_eq!(happy_nums(19), 1);
	assert_eq!(happy_nums(23), 1);
	assert_eq!(happy_nums(28), 1);
	assert_eq!(happy_nums(31), 1);
}
