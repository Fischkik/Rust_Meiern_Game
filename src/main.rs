use std::io;

fn create_wuerfel(augenzahl:Option<u16>) -> Vec<u16> {
    let augenzahl:u16  = augenzahl.unwrap_or(6);
    let mut vector_augenzahlen: Vec<u16> = Vec::new();
    for i in 0..augenzahl {
        vector_augenzahlen.push(i);
    };
    return vector_augenzahlen;
}

fn main() {

    struct Spieler {
        namen: String,
        punktzahl: i32,
        verhaltens_typ: char,
    }

    struct Wuerfel {
        augenzahl: Vec<u16>
    }

    let spieler1: Spieler = Spieler {
        namen: String::from("jan"),
        punktzahl: 0,
        verhaltens_typ: 'm'
    };

    let wuerfel1_augenzahl:Vec<u16> = create_wuerfel(None);
    let wuerfel2_augenzahl:Vec<u16> = create_wuerfel(Some(20));

    let wuerfel1: Wuerfel = Wuerfel {augenzahl: wuerfel1_augenzahl};
    let Wuerfel2: Wuerfel = Wuerfel {augenzahl: wuerfel2_augenzahl};



//     println!("Hello, world!");

//     println!("Enter your Number");

//     let mut user_guess = String::new();

//     io::stdin()
//         .read_line(&mut user_guess)
//         .expect("Failed to read line");

//     println!("You guessed {user_guess}");
}
