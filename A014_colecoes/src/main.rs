use std::collections::HashMap;

fn main() {
    vetor();
    string();
    hashmap();
}

fn vetor() {
    let lista: [u8; 5] = [1, 2, 3, 4, 5];

    println!("Valor na posição 2: {}", lista[2]);

    let mut numeros: Vec<u8> = Vec::new();

    numeros.push(1);
    numeros.push(2);
    numeros.push(3);
    numeros.push(4);
    numeros.push(5);

    println!("Valores do vetor: {:?}", numeros);

    for n in numeros {
        println!("Valor: {}", n);
    }
}

fn string() {
    let mut texto = String::from("Andre");

    texto.push_str(" Wronscki");

    println!("{}", texto);
}

fn hashmap() {
    let mut mapa: HashMap<String, &str> = HashMap::new();

    mapa.insert("nome".to_string(), "André");
    mapa.insert("url".to_string(), "https://andrewronscki.com");

    println!("{:?}", mapa);

    match mapa.get(&"url".to_string()) {
        Some(url) => println!("{}", url),
        None => println!("Não encontrou"),
    }
}
