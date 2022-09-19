pub mod another_file;


pub fn function_print(){

    println!("function print running inside {} ", "mod");
}



pub mod a{

    pub mod b{

        pub fn function_print(param: &str){
            println!("function print running inside {} {}", 
                     "mod b inside mod a from path 'path'", param);
        }

    }

}