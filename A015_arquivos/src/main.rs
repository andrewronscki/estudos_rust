use arquivo::{criar, existe, ler, ler_diretorio, obter_caminho_usuario};

mod arquivo;

fn main() {
    let caminho = obter_caminho_usuario().unwrap();

    if Ok(()) == existe(&r"C:\Users\andre\Documents\projects\estudos_rust\A015_arquivos\andre.txt")
    {
        println!("Arquivo existe");
    } else {
        println!("Arquivo não existe");
        criar(&caminho, &"andre.txt");
    }

    ler(&r"C:\Users\andre\Documents\projects\estudos_rust\A015_arquivos\andre.txt");

    match ler_diretorio(&caminho) {
        Ok(_) => println!("Leitura realizada"),
        Err(e) => println!("Erro na leitura. {}", e),
    }
}
