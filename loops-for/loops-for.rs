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
fn main() {
	for x in range(0u, 5) {
		for y in range(0u, x+1) {
			print!("*");
		}		
		println!("");
	}
}
