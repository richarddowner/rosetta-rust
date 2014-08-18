// Show how two loops may be nested within each other, with
// the number of iterations performed by the inner loop being
// controlled by the outer loop.
//
// Print this pattern:
// *
// **
// ***
// ****
// *****
use std::iter::range_inclusive;

fn loops_for() -> String {
	let mut stars = String::new();
	for i in range_inclusive(1u, 5) {
		for _ in range_inclusive(1u, i) {
			stars.push_str("*");
		}		
		stars.push_str("\n");
	}
	stars
}

#[test]
fn loops_for_test() {
	let galaxy = loops_for();	

	let mut galaxy_test = String::new();
	galaxy_test.push_str("*\n**\n***\n****\n*****\n");

	assert_eq!(galaxy, galaxy_test);
}

#[allow(dead_code)]
fn main(){}
