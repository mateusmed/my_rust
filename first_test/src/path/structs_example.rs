

struct Person{
    name: String,
    age: i32
}

impl Person {

    fn say_hi(&self) -> String{
        return format!("Meu nome é {} e eu tenho {} anos", self.name, self.age);
    }
}


fn create_person(){

    // se não converter o tipo por defaul umas string é do tipo static &str, 
    // não permitindo flexibilidade no tamanho

                              // "SpiderMan".to_string()
    let person = Person {name: String::from("SpiderMan"), age: 28};
    
    print!("{}", person.say_hi());
}