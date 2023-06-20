fn main() {
    //1.Declare uma variável inteira chamada x e atribua um valor inteiro a ela.
    let x = 1;

    //2.Declare uma variável de ponto flutuante chamada y e atribua um valor de ponto flutuante a ela.
    let y = 2.6;

    //3.Declare uma variável booleana chamada z e atribua um valor booleano a ela.
    let z = true;

    //4.Declare uma variável de caractere chamada c e atribua um caractere a ela.
    let c = 'C';

    //5.Declare uma variável de string chamada s e atribua uma string a ela.
    let s = "string";

    //6.Declare uma constante inteira chamada MAX_POINTS e atribua um valor inteiro a ela.
    const MAX_POINTS: i32 = 1;

    //7.Faça uma operação matemática com as variáveis x e y e atribua o resultado a uma nova variável.
    let result = f64::from(x) + y;

    //8.Utilize a estrutura de controle de fluxo if para verificar se a variável z é true e, se for, imprima uma mensagem.
    if z {
        println!("O valor de z é verdadeiro");
    }
    else{
        println!("O valor de z é falso");
    }        
    
    //9.Utilize a estrutura de controle de fluxo for para iterar sobre uma range de números e imprimir cada número.    
    for n in 1..11 {
        println!("{}", n);
    }

    //10.Utilize o método push() para adicionar um elemento ao final de um vetor de inteiros.
    let mut array  = vec![1,3];

    array.push(2);

    println!("{:?}", array);
}
