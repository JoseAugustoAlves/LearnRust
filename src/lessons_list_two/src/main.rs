fn main() {
    
    //1.Declare uma tupla com dois elementos, um inteiro e um ponto flutuante, e atribua valores a eles.
    
    let tuple = (5, 5.5);
    
    println!("{:?}", tuple);

    //2.Desestruture a tupla em duas variáveis separadas. 
    let (x, y) = tuple;

    println!("X: {}, Y: {}", x, y);

    //3.Acesse o segundo elemento da tupla usando o operador de indexação (.).
    
    println!("{:?}", tuple.1);

    //4.Faça um loop for sobre os elementos da tupla e imprima cada um deles.
    
    for i in tuple.iter(){
        println!("{}", i);
    }
    //5.Use a função tuple.swap(0, 1) para trocar os elementos da tupla.

    //6.Use a função tuple.len() para obter o comprimento da tupla.

    //7.Use a função tuple.is_empty() para verificar se a tupla está vazia.
    
    //8.Crie uma função que recebe uma tupla como argumento e retorna o primeiro elemento da tupla.
    
    //9.Crie uma função que recebe duas tuplas como argumentos e as compara usando o operador de igualdade (==).
    
    //10.Crie uma tupla aninhada com três elementos, onde cada elemento é uma tupla de dois elementos. 
    //Desestruture a tupla aninhada em três tuplas separadas.
}
