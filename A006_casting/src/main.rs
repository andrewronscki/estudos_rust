fn type_of<T>(_: T) -> &'static str {
	std::any::type_name::<T>()
} 

fn main() {
    let inteiro = 10;

    let int_to_float = inteiro as f32;

    let float: f64 = 2.5;

    let float_to_int = float as i32;

    println!("Valor da variável inteiro: {}, {}", inteiro, type_of(inteiro));
    println!("Valor da variável int_to_float: {}, {}", int_to_float, type_of(int_to_float));
    println!("Valor da variável float: {}, {}", float, type_of(float));
    println!("Valor da variável float_to_int: {}, {}", float_to_int, type_of(float_to_int));

		let int_to_string = inteiro.to_string();

		let string = "42";

		let string_to_int = string.parse::<i64>().unwrap();

		println!("Valor da variável int_to_string: {}, {}", int_to_string, type_of(&int_to_string));
		println!("Valor da variável int_to_string: {}, {}", string_to_int, type_of(&string_to_int));
}
