// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.


// WRITEUP
// Dans cet exercice, les variables de la structure `Book` ne possèdent pas de
// spécification de leur durée de vie.
// J'ai donc spécifié leur durée de vie avec la syntaxe `<'a>` pour le nom de la
// structure et `&'a` avant le type de chaque variables


struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
