// primitive_types6.rs
//
// Use a tuple index to access the second element of `numbers`. You can put the
// expression for the second element where ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types6` or use the `hint` watch subcommand
// for a hint.

// WRITEUP
// Dans cet exercice, il était demadné de complété l'initialisation de la
// variable `second` en utilisant la syntaxe de l'indexation des tuples.
// Je l'ai donc initialisé en indéxant `nombre` à 1 pour avoir la deuxième
// valeur du tuple


#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let second = numbers.1;

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}