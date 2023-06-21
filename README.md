# Fundamentos da Linguagem Rust

## Conceitos Fundamentais

### Declaração de Variáveis
  
  - Em Rust as variáveis são imutáveis por padrão. No entanto, podem ser mutáveis utilizando a palavra reservada `mut`. Exemplo: 
    ```Rust
    fn main() {
        let _x = 10;
        _x = 11; //erro
        let mut _y = 10;
        _y = 11; //ok
    }
    ```
  
  - A declaração de tipos é precedida pelo símbolo `:`, por exemplo `let x: i32;`. Todos os tipos primitivos estão listados na [Documentação Oficial da Linguagem](https://doc.rust-lang.org/std/index.html#primitives)
  
  - É possível sobrescrever o valor/estado de uma variável através de sua redeclaração (i.e., `shadowing`). As mudanças só permanecem dentro do escopo da nova variável declarada. Exemplo:
    ```Rust
    fn main() {
        let _x:i32 = 10;
        println!("{_x}"); //10
        scope(); //10.1
        println!("{_x}"); //10
    }

    fn scope(){
        let mut _x:f64 = 10.1;
        println!("{_x}");
    }
    ```
  
  - Constantes em Rust devem ser declaradas no escopo global e com a devida tipagem. Exemplo:
    ```Rust
    const PI:f64 = 3.1415926535897932384626433;

    fn main() {
        println!("{PI}");
    }
  
  - Structs podem ser declaradas utilizando a palavra reservada `struct`. A instanciação de structs é feita invocando o nome da struct seguido dos valores dos atributos. Os atributos podem ser acessados utilizando o operador `.`. Exemplo:
    ```Rust
    struct Pessoa {
        nome : String,
        sobrenome : String,
    }
    
    fn main(){
        let _pessoa : Pessoa = Pessoa { nome : "Sara".to_owned(), sobrenome : "Nahra".to_owned()};
        println!("{}", pessoa1.nome)
    }
    ```
    
### Declaração de Funções
    
  - A declaração de funções é feita a partir da palavra reservada `fn` seguida do nome da função, dos parâmetros entre parenteses `()` e do retorno esperado `->`. Funções podem ser invocadas através do nome seguido de parenteses. Exemplo:
    ```Rust
    fn add(x: i32, y: i32) -> i32 {
        x + y //sem a palavra reservada return
        //return x + y
    }
    
    let sum = add(5, 10); //15
    ```
  - Funções do tipo método são declaradas dentro de escopos definidos por `impl <struct/classe> {...}`. A invocação de métodos é feita com o operador `.`. Exemplo:
    ```Rust
    struct Pessoa {
        nome: String,
        sobrenome: String,
    }

    impl Pessoa {
        fn nome_completo(&self) -> String {
            return format!("{} {}", self.nome, self.sobrenome);
        }
    }

    fn main(){
        let pessoa : Pessoa = Pessoa { nome : "Sara".to_owned(), sobrenome : "Nahra".to_owned()};
        println!("{}", pessoa.nome_completo());
        println!("{}", Pessoa::nome_completo(&pessoa)); //mesmo  resultado
    }
    ```
    > Métodos também podem ser acessados estaticamente através do nome do tipo seguido do operador `::` e o nome do método. Exemplo de Construtor
    ```Rust
    impl Pessoa {
        fn new(_nome : String, _sobrenome : String) -> Pessoa {
            return Pessoa { nome = _nome, sobrenome = _sobrenome }
        }
    }  
    ```
  - [_Closure functions_](https://doc.rust-lang.org/book/ch13-01-closures.html) são funções tratadas como variáveis definidas dentro de um escopo, existindo apenas dentro dele. Esse conceito é denominado "funcões como cidadãos de primeira classe" (vide paradigma funcional).
    ```Rust
    fn main(){
        if( true ){
            let x = | y : i32 | y + 2;
            println!("{}",x(10));
        }
        println!("{}",x(10)); //a função não existe fora do escopo
    }
    ```
 - _Generator functions_ são funções que geram iteradores. São definidos pelas palavras reservadas `yield` e `move`. 
    ```Rust
    fn main() {
        fn count_up_from(start: i32) -> impl Iterator<Item = i32> {
            (start..).into_iter()
        }
    }
    ```

### Importação de Crates com Cargo
  - Para importar crates com cargo basta inserir a dependência no arquivo cargo.toml e declarar com `use` no arquivo `.rs`. Exemplo:
    
    > No arquivo cargo.toml
    ```toml
    [package]
    name = "hello_sara"
    version = "0.1.0"
    edition = "2021"

    [dependencies]
    num = "0.4"
    ```
    
    > No arquivo .rs
    ```Rust
    use num::complex::Complex;
    
    fn main(){
        let x : Complex<f64> = Complex { re: 2.1, im: -1.2}; //todo tipo possui essa sintaxe para inicialização
        let y : Complex<f64> = Complex::new(11.0, 10.1); //implementação do método estático new
        let resultado = x + y;
        println!("{} + {}i", resultado.re, resultado.im);
    } 
    ```
  - A importação de crates de terceiros também pode ser feita através do comando `cargo add` no terminal. Exemplo: 
    ```sh 
    cargo add num
    ```

### Operações com Números

  - Números em Rust não são convertidos automaticamente (e.g.: i16 != i32).
  - Números em Rust possuem métodos, porém exigem tipagem definida. Exemplo:
    ```Rust
    fn main(){
        let x :f64 = 29_f64;
        let y = x.floor();
        let z = 29.99999_f64.floor();
        println!("{}", y);
        println!("{}", z);
    }
    ```
 - A declaração de números inteiros em diferentes bases pode ser feita através do prefixo `0b` (base 2), `0o` (base 8), `0x` (base 16). Exemplo:
    ```Rust
    fn main(){
        let base2 = 0b11_i32;
        let base8 = 0o7_i32;
        let base16 = 0xf_i32;
        let base10 = 16777215;
        println!("{} {} {} {}", base2, base8, base16, base10);
        println!("{:b} {:b} {:b} {:b}", base2, base8, base16, base10);
        println!("{:o} {:o} {:o} {:o}", base2, base8, base16, base10);
        println!("{:x} {:x} {:x} {:x}", base2, base8, base16, base10);
    }
    ```
 - Análogo a outras linguagens, as operações lógicas `>`, `>=`, `==`, `!=`, `<` e `<=` são suportadas.

### Laços de Repetição
  - É possível iterar elementos de um tipo iterável através do `for`. Exemplo de loop for em Rust:
    ```Rust
        fn main(){
            let iteravel = [1, 2, 3, 4, 5];

            //Quando é necessário iterar por todos os objetos do iterável
            for item in iteravel {
                //...
            }

            //Quando é necessário iterar pelo índice dos objetos
            //Atenção: recuperar valores a partir de seus índices é mais custoso
            for i in 0..iteravel.len() {
                //...    
            }

            //Quando não é necessário criar uma variável para o escopo
            for _ in 0..iteravel.len() {
                //...
            }

            //Laço for com continue
            for i in 0..10 {
                if i % 2 == 0 { //% é o resto da divisão
                    continue; //salta para a próxima iteração
                } 
                println!("{}", i); //1, 3, 5, 7, 9
            }
        }
    ```
  - Também é possível iterar elementos de um tipo iterável através do `while`. Exemplo de loop while em Rust:
    ```Rust
        use std::time::{Duration,Instant};

        fn main(){
            let mut count = 0;
            let tf = Duration::new(1,0);
            let t0 = Instant::now();
            while (Instant::now() - t0) < tf {
                count = count + 1; 
            }
            println!("{}", count);
        }
    ```
    
- Em Rust também é possível obter ainda mais controle utilizando `loop`. A execução ocorrerá até que o ciclo seja interrompido com a palavra reservada `break`. Exemplo:
    ```Rust
        fn main(){
            let mut i = 0;
            loop {
                i = i + 1;
                if i > 10 {
                    break;
                }
            }
        }
    ```
    
    # Colaboradores
    - Caio Augusto Palma
    - Sara Robert Nahra
    
    # Referências
    - [A linguagem de programação Rust](https://rust-br.github.io/rust-book-pt-br/)
    
