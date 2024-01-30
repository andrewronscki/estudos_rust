use std::{
    env,
    ffi::OsStr,
    fs::{self, metadata, File},
    io::Read,
    path::PathBuf,
};

pub fn ler_diretorio(caminho: &str) -> Result<(), std::io::Error> {
    let items = fs::read_dir(caminho)?;

    for item in items {
        let item = item?;

        let item_caminho = item.path();

        if item_caminho.is_dir() {
            println!("Diretório: {}", item_caminho.display());
        } else {
            println!("Arquivo: {}", item_caminho.display());
        }
    }

    Ok(())
}

pub fn existe(caminho_completo: &str) -> Result<(), &'static str> {
    if metadata(caminho_completo).is_ok() {
        Ok(())
    } else {
        Err("O arquivo não existe.")
    }
}

pub fn ler(caminho_completo: &str) {
    if existe(&caminho_completo).is_ok() {
        match File::open(&caminho_completo) {
            Ok(mut arquivo) => {
                let mut conteudo = String::new();

                arquivo.read_to_string(&mut conteudo).unwrap();

                println!("Arquivo aberto: {}", conteudo);
            }
            Err(e) => println!("Erro ao ler o arquivo: {}", e),
        }
    }
}

pub fn obter_caminho_usuario() -> Option<String> {
    match env::current_dir() {
        Ok(caminho) => Some(pathbuf_para_string(caminho).unwrap()),
        Err(e) => {
            eprintln!("Erro ao obter o caminho do diretório do projeto: {}", e);
            None
        }
    }
}

pub fn criar(caminho: &str, nome_arquivo: &str) {
    let caminho_completo = format!(r"{}\{}", caminho, nome_arquivo);

    match File::create(&caminho_completo) {
        Ok(_) => println!("Arquivo criado com sucesso no caminho {}", caminho_completo),
        Err(e) => println!("Erro ao criar o arquivo. {:?}", e),
    };
}

fn pathbuf_para_string(caminho: PathBuf) -> Option<String> {
    let os_str: &OsStr = caminho.as_os_str();

    match os_str.to_str() {
        Some(valid_str) => Some(valid_str.to_owned()),
        None => {
            println!("O caminho contém caracteres inválidos para UTF-8.");
            None
        }
    }
}
