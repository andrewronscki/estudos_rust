// pascal case
enum Fruta {
    Maca,    // Maca = 0
    Banana,  // Banana = 1
    Morango, // Morango = 2
    Acai,    // Acai = 3
}

enum Coordenada {
    DoisD(i32, i32),
    TresD(i32, i32, i32),
}

// pascal case
struct Pessoa {
    nome: String,
    idade: i8,
    altura: f32,
}

// O mais perto de trabalhar com classes
struct Retangulo {
    altura: u32,
    largura: u32,
}

// impl Retangulo {
//     fn calcular_area(&self) -> u32 {
//         self.largura * self.altura
//     }

//     fn new(largura: u32, altura: u32) -> Self {
//         Self { altura, largura }
//     }
// }

// Trait = semelhante a interfaces e como implementar a Trait
trait FormaGeometrica {
    fn calcular_area(&self) -> u32;
    fn new(largura: u32, altura: u32) -> Self;
}
impl FormaGeometrica for Retangulo {
    fn calcular_area(&self) -> u32 {
        self.largura * self.altura
    }

    fn new(largura: u32, altura: u32) -> Self {
        Self { altura, largura }
    }
}

fn main() {
    enumeracao(Fruta::Maca);
    enumeracao(Fruta::Banana);
    enumeracao(Fruta::Morango);
    enumeracao(Fruta::Acai);
    enumeracao_com_dados();
    estrutura();
}

fn enumeracao(fruta: Fruta) {
    match fruta {
        Fruta::Maca => println!("É uma maçã."),
        Fruta::Banana => println!("É uma banana."),
        Fruta::Morango => println!("É um morango."),
        Fruta::Acai => println!("É um açaí."),
    }
}

fn enumeracao_com_dados() {
    let ponto2d = Coordenada::DoisD(5, 10);
    let ponto3d = Coordenada::TresD(3, 8, 15);

    match ponto2d {
        Coordenada::DoisD(x, y) => println!("Coordenada 2d: {}, {}", x, y),
        Coordenada::TresD(_, _, _) => println!("Coordenada 3d"),
    }

    match ponto3d {
        Coordenada::DoisD(_, _) => println!("Coordenada 2d"),
        Coordenada::TresD(x, y, z) => println!("Coordenada 3d: {}, {}, {}", x, y, z),
    }
}

fn estrutura() {
    let andre = Pessoa {
        altura: 1.68,
        idade: 28,
        nome: String::from("André Wronscki"),
    };

    println!("Nome: {}", andre.nome);
    println!("Idade: {}", andre.idade);
    println!("Altura: {}", andre.altura);

    let retangulo1 = Retangulo {
        largura: 10,
        altura: 20,
    };

    let area1 = retangulo1.calcular_area();

    println!("Retângulo 1 {}", area1);

    let retangulo2 = Retangulo::new(10, 30);

    let area2 = retangulo2.calcular_area();

    println!("Retângulo 2 {}", area2);
}
