use crate::stack::Stack;

pub fn add(stk: &mut Stack) {
	let val1 = stk.pop();
	let val2 = stk.pop();

	stk.push(val1+val2);
}

pub fn sub(stk: &mut Stack) {
	let val1 = stk.pop();
	let val2 = stk.pop();

	stk.push(val1-val2);
}

pub fn mult(stk: &mut Stack) {
	let val1 = stk.pop();
	let val2 = stk.pop();

	stk.push(val1*val2);
}

pub fn modulo(stk: &mut Stack) {
	let val1 = stk.pop();
	let val2 = stk.pop();

	stk.push(val1%val2);
}
