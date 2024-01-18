// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.


// WRITEUP
// Le but de cet exercice était de remplacer le type `u32` en un type générique.
// Pour cela j'ai créé un type générique `T` pour qu'il contenir une valeur de
// n'importe quel type. Pour ajouter un type générique, il ne faut pas oublier
// d'ajouter `<T>>` après la déclaration de l'implémentation, après nom de cette
// implémentation et après le nom de la structure.


struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
