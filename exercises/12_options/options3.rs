// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.


// WRITEUP
// Il y a une erreur au niveau de ka variable `p` à cette ligne :
// Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
// car la variable est déplacé et réutilisé plus tard et comme elle est de type
// `Point`, il n'est pas possible d'implémenter le trait `Copy`.
// Pour résoudre cela il faut utilisé le mot clé `ref` afin d'emprunté la valeur
// plutôt que de la déplacé.


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
