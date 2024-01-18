// structs3.rs
//
// Structs contain data, but can also have logic. In this exercise we have
// defined the Package struct and we want to test some logic attached to it.
// Make the code compile and the tests pass!
//
// Execute `rustlings hint structs3` or use the `hint` watch subcommand for a
// hint.


// WRITEUP
// Pour cet exercice, il nous était demandé de complèter les fonctions
// `is_international` et `get_fees` 
// Pour `is_international`, la fonction doit retourner un booléen si le pays
// émetteur est le même que le pays destinataire du colis ou non et a pour
// argument `&self`. 
// J'ai d'abord récupéré les variables `sender_country` et `recipient_country`
// grâce à `&self`, je les ai comparé et retourné faux si les pays étaient les
// même et vrai s'ils étaient diférents
// Pour la fonction `get_fees`, j'ai dans un premier temps récupérer le poid en
// grammes du colis grâce au `&self` mis en argument de la fonction et j'ai
// retourné le calcul des frais de ports en multipliant la masse et le prix par
// gramme. 
// Un `u32` était attendu pour le retour de la fonction étant donné que la
// variable `cents_per_gram` mis en argument était définie en `u32`.

#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: u32,
}

impl Package {
    fn new(sender_country: String, recipient_country: String, weight_in_grams: u32) -> Package {
        if weight_in_grams < 10 {
            // This is not how you should handle errors in Rust,
            // but we will learn about error handling later.
            panic!("Can not ship a package with weight below 10 grams.")
        } else {
            Package {
                sender_country,
                recipient_country,
                weight_in_grams,
            }
        }
    }

    fn is_international(&self) -> bool {
        // Something goes here...
        let sender = &self.sender_country;
        let recipient = &self.recipient_country;
        if sender == recipient {
            false
        } else {
            true
        }
    }

    fn get_fees(&self, cents_per_gram: u32) -> u32 {
        // Something goes here...
        let weight = &self.weight_in_grams;
        weight*cents_per_gram
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Austria");

        Package::new(sender_country, recipient_country, 5);
    }

    #[test]
    fn create_international_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Russia");

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(package.is_international());
    }

    #[test]
    fn create_local_package() {
        let sender_country = String::from("Canada");
        let recipient_country = sender_country.clone();

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(!package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Spain");

        let cents_per_gram = 3;

        let package = Package::new(sender_country, recipient_country, 1500);

        assert_eq!(package.get_fees(cents_per_gram), 4500);
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000);
    }
}
