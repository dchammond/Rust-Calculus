# Rust-Calculus
Just messing around with evaluating mathematical expressions and performing integrals/derivatives.

1. User inputs expression as a string
2. Multiple Passes:
	1. Strip Whitespace
	2. Transform into a Vector of Tokens
	3. Simplify
3. Several things it can do:
	1. Simply output simplified expression
	2. Evaluate if no variables Or if variables are given values as program arguments
	3. Perform analysis as requested by program arguments
		1. Symbolic Integration (HARD)
		2. Symbolic Differential (NOT AS HARD)
		3. Definite Integral
		4. Definite Differential
		5. Graph (AT THE END)