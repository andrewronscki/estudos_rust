use operacoes::matematica::{somar, subtrair};
use rand::random;

mod operacoes;

fn main() {
    println!("2+2={}", somar(2, 2));
		println!("2-2={}", subtrair(2, 2));

		println!("Número aleatório: {}", random::<i8>())
}
