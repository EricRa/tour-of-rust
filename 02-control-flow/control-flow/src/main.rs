fn main() {
    // CHAPTER 2 - BASIC CONTROL FLOW

    // if/else

    // all usual operators are valid:
    // == , != , < , > , <= , >= , ! , || , &&

    let x = 42;
    if x < 42 {
        println!("less than 42");
    } else if x == 42 {
        println!("is 42");
    } else {
        println!("greater than 42");
    }

    // Loops (infinite)

    let mut x = 0;
    loop {
        x += 1;
        if x == 42 {
            break;
        }
    }
    println!("{}", x);

    // While Loops

    // if condition evaluates to false, the loop will exit
    let mut x = 0;
    while x != 42 {
        x += 1;
    }
    println!("x is {}", x);
	
	
	// For Loops
	
	// the .. operator creates an iterator that generates numbers
	// from a start number up to but no including an end number
	
	// The ..= operator creates an iterator that generates numbers
	// from a start number up to an including an end number
	
	for x in 0..5 {
		println!("{}", x);
	}
	
	for x in 0..=5 {
		println!("{}", x);
	}
	
	
	// Match
	
	// like a switch statement
	// match is exhaustive, so all cases must be handled
	
	// matching combined with destructuring is by far one of the most
	// common patterns you will see in Rust
	
	let x = 42;
	
	match x {
		0 => {
			println!("found zero");
		}
		// we can match against multiple values
		1 | 2 => {
			println!("found 1 or 2!");
		}
		// we can match against ranges
		3..=9 => {
			println!("found a number 3 to 9 inclusive");
		}
		// we can bind the matched number to a variable
		matched_num @ 10..=100 => {
			println!("found {} between 10 to 100!", matched_num);
		}
		// this is the default match that must exist 
		// if not all cases are handled:
		_ => {
			println!("found something else");
		}
	}

	// Returning Values From Loop
	
	// loop can break to return a value
	
	let mut x = 0;
	let v = loop {
		x += 1;
		if x == 13 {
			break "found the 13";
		}
	};
	println!("from loop: {}", v);
	
	// Returning Values From Block Expressions
	
	// If the last statement in an if, match, function, or scope block
	// is an expression without a ;, Rust will return it as a value
	// from the block.  This is a great way to create concise logic
	// that returns a value that can be put into a new variable
	
	fn example() -> i32 {
		let x = 42;
		// Rust's ternary expression:
		let v = if x < 42 { -1 } else { 1 };
		println!("from if: {}", v);
		
		let food = "hamburger";
		let result = match food {
			"hotdog" => "is hotdog",
			// notice the braces are optional when it's just a single
			// return expression
			_ => "is not hotdog",
		};
		println!("identifying food: {}", result);
		
		let v = {
			// This scope block lets us get a result without
			// polluting function scope
			let a = 1;
			let b = 2;
			a + b
		};
		println!("from block: {}", v);
		
		// The idiomatch way to return a value in rust from a function
		// at the end
		v + 4
		
	}
	
	example();
	
	
	
	
	
	
}


