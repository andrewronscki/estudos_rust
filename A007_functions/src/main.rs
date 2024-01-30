fn main() {
    nome_da_funcao();

    let a = with_return();

    println!("O valor com retorno é: {}", a);

    let b = without_return();

    println!("O valor sem retorno é: {}", b);

    println!(
        "O maior valor entre {} e {} é: {}",
        10,
        20,
        maior_valor(10, 20)
    );

    let d = 5;

    let increment_result = increment(d);

    println!(
        "Número original: {}, Número incrementado: {}",
        d, increment_result
    );
}

fn increment(mut a: i32) -> i32 {
    a += 1;
    a
}

fn nome_da_funcao() {
    println!("Hello, world!");
}

fn with_return() -> i8 {
    return 3;
}

fn without_return() -> i8 {
    3
}

fn maior_valor(a: i32, b: i32) -> i32 {
    {
        if a > b {
            a
        } else {
            b
        }
    }
}
