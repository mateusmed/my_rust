mod path;

use path::a;
use path::a::b as xpto;


fn variables_verify(){

    let mut a = 1;
    a = a + 1;

    println!("value of a is {}", a);

}


fn vector_verify(){

    let b = vec![1, 2, 3];              // ! -> macro | argumentos nomeados só funcionam nas macros

    println!("vector - pos vec {}", b[0]);      // ! -> macro | argumentos nomeados só funcionam nas macros
                                                // exemplo abaixo
    println!("vector - pos vec {pos_zero}", pos_zero=b[0]);
}

fn atribuition_verify(){

    //let mut c = vec![1, 2, 3];
    //let d = c;

    // pq isso não é possivel?
    //println!("atribuition - pos vec {}", c[0]);

    let mut a = 5;
    let mut b = a;

    a = a + 1;
    b = b - 1;

    println!("atribuition - value a {} value b {}", a, b);

}

// rust não possui nulo, 
fn function_return_value() -> i32 {

    let _my_string = "my string for infence type";     
    let _my_string_explicit: &str = "my string with explicite type";  

    let _my_number = 32;
    let _my_number_explicit: i32 = 1234567;

    // só é necessário colocar o _ no prefixo da variavel se não está utilizando
    // é uma maneira de não propagar "worning"

    return _my_number;
}

fn sum(x: i32, y: i32) -> i32{

    return x + y;
}

fn sum_with_new_style_return(x: i32, y: i32) -> i32{
    x + y       // se a ultima linha de uma função não tiver return nem ponto e virgula, ela será retornada
}


fn main() {

    path::function_print();
    path::another_file::function_print();
    
    path::a::b::function_print("ok?");
    a::b::function_print("call to test 'use'");
    xpto::function_print("rename as xpto");

    variables_verify();
    vector_verify();
    atribuition_verify();
    function_return_value();
    
    let response_value = sum(3, 5);
    println!("my response value {}", response_value);


    let response_return_new_style = sum_with_new_style_return(1, 2);
    println!("response_return_new_style {}", response_return_new_style);

    println!("Hello, world!");
}
