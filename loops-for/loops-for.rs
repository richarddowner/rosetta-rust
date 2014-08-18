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

fn loops_for() {
	for i in range_inclusive(1u, 5) {
		for _ in range_inclusive(1u, i) {
			print!("*");
		}		
		println!("");
	}
}

#[allow(dead_code)]
fn main() {
	loops_for()
}

#[test]
fn loops_for_test() {
	loops_for();	
}
