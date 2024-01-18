// if3.rs
//
// Execute `rustlings hint if3` or use the `hint` watch subcommand for a hint.

// WRITEUP
// Dans ce code, la fonction animal_habitat associe un habitat un à l'animal
// identifié. Pour qu'une condition si fonctionne, il faut qu'elle retourne
// le même type de variable. Or, dans la fonction animal_habitat, la variable
// identifier dans être un entier pour être utilisé dans la condition de la
// variable habitat. Sauf que `if animal == "gopher"` retourne `2.0` et le
// dernier sinon retourne `"Unknown"`. Pour corriger cela, j'ai remplacé `2.0`
// par `2` pour retourner un entier à la condition `if animal == "gopher"` et
// j'ai remplacé `"Unknown"` par `4` pour retourner le dernier sinon, mais un
// autre entier tel que 0 ou 8 aurait aussi fonctionné.


pub fn animal_habitat(animal: &str) -> &'static str {
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2
    } else if animal == "snake" {
        3
    } else {
        4
    };

    // DO NOT CHANGE THIS STATEMENT BELOW
    let habitat = if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    };

    habitat
}

// No test changes needed.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}


