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
    
    # Colaboradores
    - Caio Augusto Palma
    - Sara Robert Nahra
    
    # Referências
    - 
    
