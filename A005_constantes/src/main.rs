fn main() {
		const COMPANY_NAME: &str = "ANDRE WRONSCKI LTDA";

    let mut name = "André";
    println!("{}", name);

    name = "André2";
    println!("{}", name);

		let company_name = "ANDRE WRONSCKI LTDA";

		println!("Valor da constante é: {}", COMPANY_NAME); 
		// ao buildar ele altera a constante e escreve o valor ex:
		// println!("Valor da constante é: {}", "ANDRE WRONSCKI LTDA");
		println!("Valor da constante é: {}", company_name); // a variável está em memória, então irá ir na memória pegar o valor dela e atribuir onde é usado.
}
