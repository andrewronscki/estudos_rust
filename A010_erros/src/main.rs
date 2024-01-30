use std::{
    fs::File,
    io::{self, Read},
    panic,
};

fn main() {
    // fecha a execução da aplicação
    // funcao_com_panic(0);

    // tratamento de erros panics para não fechar a execução da aplicação
    let resultado = std::panic::catch_unwind(|| {
        let a = funcao_com_panic(0);

        Ok::<i32, &str>(a)
    });

    match resultado {
        Ok(valor) => {
            println!("O resultado é: {}", valor.unwrap())
        }
        Err(_) => println!("A função resultou em panic"),
    }

    // result type
    let resultado =
        ler_arquivo(r"C:\Users\andre\Documents\projects\estudos_rust\A010_erros\src\main2.rs");

    match resultado {
        Ok(conteudo) => {
            println!("Conteúdo do arquivo: {}", conteudo);
        }
        Err(erro) => {
            println!("Erro ao ler arquivo: {}", erro);
        }
    }

    // option type
    let resultado_divisao = dividir(100 as f64, 0 as f64);

    match resultado_divisao {
        Some(resultado) => {
            println!("O resultado da divisão é igual a {}", resultado)
        }
        None => {
						println!("Não foi possível fazer a divisão")
				},
    }
}

fn funcao_com_panic(valor: i32) -> i32 {
    if valor == 0 {
        panic!("O valor não pode ser zero");
    }

    valor
}

fn ler_arquivo(caminho: &str) -> Result<String, io::Error> {
    let mut arquivo = File::open(caminho)?;

    let mut conteudo = String::new();

    let _ = arquivo.read_to_string(&mut conteudo);

    Ok(conteudo)
}

fn dividir(dividendo: f64, divisor: f64) -> Option<f64> {
    if divisor == 0.0 {
        None
    } else {
        Some(dividendo / divisor)
    }
}
