// enums3.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint enums3` or use the `hint` watch subcommand for a
// hint.


// WRITEUP
// Dans ce code il nous est demandé de remplir l'enumération `Message` et la
// fonction `process`.
// J'ai rempli l'enumération `Message` en me basant sur la structure `State`
// Puis pour la fonction `process`, j'ai fait un match de `message` comme
// demandé pour y traiter les différents types de messages défini dans
// l'enumération `Message` en les associant aux bonnes fonctions présentes dans
// l'implémentation `States`.
// On peut noter qu'un dérérencement de la variable `s` est nécessaire pour
// récupérer la chaine de caractères de la méthode `echo` tout comme sa
// convertion en String car cette méthode est appelé avec cette nouvelle String.
// De plus, il ne faut pas oublier la synthaxe du tuple avec les doubles
// parenthèses pour l'argument de la méthode change_color.


enum Message {
    // TODO: implement the message variant types based on their usage below
    Quit,
    Echo(String),
    Move(Point),
    ChangeColor(u8, u8, u8),
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
    message: String,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&mut self, s: String) {
        self.message = s
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        // TODO: create a match expression to process the different message variants
        // Remember: When passing a tuple as a function argument, you'll need extra parentheses:
        // fn function((t, u, p, l, e))
        match message {
            Message::Quit => self.quit(),
            Message::Echo(s) => self.echo((*s).to_string()),
            Message::Move(p) => self.move_position(p),
            Message::ChangeColor(r, g, b) => self.change_color((r, g, b)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
            message: "hello world".to_string(),
        };
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
        assert_eq!(state.message, "Hello world!");
    }
}


