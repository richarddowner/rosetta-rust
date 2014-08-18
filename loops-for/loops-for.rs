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
fn loops_for() {
	for x in range(0u, 5) {
		for _ in range(0u, x+1) {
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
