fn concatenate_string(s1: &str , s2: &str) -> String{
	let mut result = String::from ("");
	result.push_str(s1);
	result.push_str(s2);
	result
}




fn main() {
	let text = String::from ("Hello");
	let string1 = &text[0..3];
	let string2 = &text[3..];
	let concatenated = concatenate_string(&string1, &string2);

	println!("{}", concatenated);

}
