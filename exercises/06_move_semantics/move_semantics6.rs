// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

// WRITEUP
// Dans ce code, il nous est demandé d'ajouter ou de supprimer des références.
// J'ai ajouté une référence `.clone` à `date` dans l'appel de la fonction 
// get_char pour préserver la variable `data` dans la fonction main qui sera 
// réutiliser dans l'appel de la fonction `string_uppercase`.
// J'ai supprimé la référence `.to_uppercase()` dans la fonction `string_uppercase`
// car il y a un conflit avec le lifetime de la variable `&data` qui est une
// référence à la variable `data` et donc une variable temporaire qui sera
// suprimé à la fin de l'instruction.

fn main() {
    let data = "Rust is great!".to_string();

    get_char(data.clone());

    string_uppercase(&data);
}

// Should not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: &String) {
    data = &data.to_uppercase();

    println!("{}", data);
}
