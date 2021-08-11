use crate::stack::Stack;

pub fn add(stk: &mut Stack) {
	let val1 = stk.pop();
	let val2 = stk.pop();

	stk.push(val2 + val1);
}

pub fn sub(stk: &mut Stack) {
	let val1 = stk.pop();
	let val2 = stk.pop();

	stk.push(val2 - val1);
}

pub fn mult(stk: &mut Stack) {
	let val1 = stk.pop();
	let val2 = stk.pop();

	stk.push(val2 * val1);
}

pub fn modulo(stk: &mut Stack) {
	let val1 = stk.pop();
	let val2 = stk.pop();

	stk.push(val2 % val1);
}

pub fn div(stk: &mut Stack) {
	let val1 = stk.pop();
	let val2 = stk.pop();

	stk.push(val2 / val1);
}

pub fn switch(stk: &mut Stack) {
	let val1 = stk.pop();
	let val2 = stk.pop();

	stk.push(val1);
	stk.push(val2);
}
