fn main() {
    // let mut -> permite alterar a variável
    // let -> não permite alterar a variável

    let texto = String::from("André"); // "" -> string estática
		let texto2 = texto;
    println!("Hello, {}!", texto2); // texto2 atribuiu o valor de texto e a variável texto foi destruída

		let mut valor = 50;
		let referencia = &valor; // referencia atribuiu o valor de valor e a variável valor não foi destruída, pois foi passado somente a referência
		
		println!("Valor, {0} e {1}!", valor, referencia);
}
