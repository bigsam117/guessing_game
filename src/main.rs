// libreria standard per poter usare input output
use std::io;

// funzione principale
fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // "let mut" inizializza una variabile mutabile
    // ogni variabile per default sono immutabili(costante)
    // quindi si usa la keyword "mut" per renderla mutabile
    let mut guess = String::new();
    // String::new() è una funzione che crea una nuova stringa
    // il "::" in "::new" indica che "new" è associato all
    // tipo String, invece che a una particolare instanza di
    // String. Viene chiamato anche metodo statico.
    // la funzione "new" crea una nuova stringa vuota.

    // la funzione "io::stdin()" serve a leggere un input
    // mentre la funzione read_ine() ha la funzione di 
    // prendere l'input e metterlo dentro una variabile
    // in questo caso "mut guess" con la referenza "&"
    io::stdin().read_line(&mut guess)    
        .expect("Failed to read line");
    // ".expect()" gestisce le eccezioni in caso di qualche
    // errore. quando si richiama un metodo ".foo()" si tende
    // ad andare a capo in nuova linea

    println!("You guessed: {}", guess);
}
