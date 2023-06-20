fn main() {
    let p1: Pessoa = Pessoa{
        nome:String::from("Caio"), 
        documento:Documento{
            numero:String::from("12345678910")
        }
    };
    let p2 = p1.clone();
    println!("{} {}", p1.to_string(), p2.to_string())
}

#[derive(Clone)]
struct Pessoa {
    nome: String,
    documento: Documento,
}

#[derive(Clone)]
struct Documento {
    numero: String,
}

impl Pessoa {
    fn to_string(&self) -> String {
        return format!("{} {}", self.nome, self.documento.to_string());
    }
}

impl Documento {
    fn to_string(&self) -> String {
        return format!("{}", self.numero);
    }
}