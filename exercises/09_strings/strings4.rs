// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


// WRITEUP
// Dans cet exercice, il fallait ajouter `string_slice` si le paramètre est un
// `&str` ou `string` si le paramètre est un `String`.
// Pour `"blue"`, c'est un `&str` donc je lui ai ajouté `string_slice`.
// Pour `"red".to_string()`, il y a une conversion avec `.to_string()` donc
// c'est un `String`.
// Pour `String::from("hi")`, il y a une conversion avec `String::from()` donc
// c'est un `String`.
// Pour `"rust is fun!".to_owned()`, il y a un `to_owned()` qui crée une copie
// d'un `&str` vers un `String`, donc c'est un `String`.
// Pour `"nice weather".into()`, il y a un `.into()` qui converti un entier en
// `String`, donc c"est un `String`.
// Pour `format!("Interpolation {}", "Station")`, il y a la macro `format!()`
// qui crée une nouvelle chaine de caratères `String` a partir de ses arguments,
// donc c'est un `string`.
// Pour `&String::from("abc")[0..1]`, il y a une plage d'indice allant de 0 à 1,
// c'est donc un `&str`.
// Pour `"  hello there ".trim()`, il y a un `.trim()` qui renvoie un `&str`
// après avoir supprimé les espaces en trop, c'est donc un `&str`.
// Pour `"Happy Monday!".to_string().replace("Mon", "Tues")`, il y a une
// conversion avec `.to_string()` et `.replace()` renvoie un `String` après avoir
// remplacé un élément donc c'est un `String`.
// Pour `"mY sHiFt KeY iS sTiCkY".to_lowercase()`, il ya un `.to_lowercase()`
// qui renvoie un `String` après avoir mis tous les caractères de la chaine en
// minuscule, c'est donc un `String`.


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
