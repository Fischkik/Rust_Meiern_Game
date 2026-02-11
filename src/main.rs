use std::io::{self, BufRead};

/*  Spiel regeln 
    Es gibt zwei 6 seitige wuerfel 
    Es wird einmal pro spieler geworfen 
    Es wird einmal pro spieler angesagt 
    Es wird einamal pro spieler angezweifelt oder nicht angezweifelt 
    Die aktuelle zahl muss hoeher sein als die zuletzt angesagte zahl
    Wird Jemand biem Luegen erwischt dann bekommt ehr einen Minus punkt 
    Zweifelt jemand an aber die ansage war richtig so verliert der Zweifeler einen Punkt
    Niedriger luegen ist erlaubt 
    21 ist die Hoechste zahl
    Pashe sind darunter */
fn create_game() {
    let mut player_count_str:String = String::new();
    println!("Enter the number of Players");
    io::stdin()
        .read_line(&mut player_count_str)
        .expect("Failed procesing user input for Player count");
    let player_count: u32 = player_count_str.trim().parse().expect("Failed to convert the user inpu into an integar");
    for i in 0..player_count {

        // --- Read name ---
        println!("Enter name:");
        let mut str_name = String::new();
        io::stdin().read_line(&mut str_name).unwrap();
        let name = str_name.trim().to_string();

        // --- Read behavior ---
        println!("Enter behavior (v)orsichtig / (m)utig / (n)ormal:");
        let mut str_behavior = String::new();
        io::stdin().read_line(&mut str_behavior).unwrap();
        let verhaltens_typ= str_behavior.trim().chars().next();
        verhaltens_typ = verhaltens_typ.expect("Invalid behavior");

        // --- Read bool ---
        println!("Is this player a CPU? (true/false):");
        let mut str_ist_bot = String::new();
        io::stdin().read_line(&mut str_ist_bot).unwrap();
        let ist_bot: bool = str_ist_bot.trim().parse().unwrap_or(false);

        // --- Call the function ---
        let value = create_player(name, verhaltens_typ, ist_bot);

        // --- Append result to list ---
        results.push(value);
    }

        
}

fn create_player(name:String, verhaltens_typ:char, ist_bot:bool){}

fn play_game(spieler_liste:&mut Vec<Spieler>, wuerfel_liste:&[Wuerfel; 3]){}

fn fninish_game(spieler_liste:&mut Vec<Spieler>, wuerfel_liste:&[Wuerfel; 3]) {}

fn anzweifeln(wurf:&u8, ansage:&u8) -> bool{}

fn ansagen(moegliche_werte:&[u8;21]) -> u8{}

fn  wuerfel_wurf(moegliche_werte:&[u8;21]) -> u8{}

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
        ist_bot: bool,
        verhaltens_typ: char 
    }

    struct Wuerfel {
        augenzahl: Vec<u16>
    }

    let moegliche_wuerfe_array: [u8;21] = [ 
        31, 32, 41, 42, 43, 51, 
        52, 53, 54, 61, 62, 63,
        64, 65, 11, 22, 33, 44, 
        55, 66, 21];
        
    let wuerfel1_augenzahl:Vec<u16> = create_wuerfel(None);
    let wuerfel2_augenzahl:Vec<u16> = create_wuerfel(None);

    let wuerfel1: Wuerfel = Wuerfel {augenzahl: wuerfel1_augenzahl};
    let wuerfel2: Wuerfel = Wuerfel {augenzahl: wuerfel2_augenzahl};

    println!("{}", "-".repeat(20));
    println!("If you wich to start the game press enter");
    io::stdin().lock().lines().next();
    println!("Initilizing game");
    create_game();


    // Test cases
    // richtige wuerfe mit richtiger range und wertung 
    // richtige spieler erstellung 
    // richtige rechnung der punkte 
    // richtiger ablauf 


//     println!("Hello, world!");

//     println!("Enter your Number");

//     let mut user_guess = String::new();

//     io::stdin()
//         .read_line(&mut user_guess)
//         .expect("Failed to read line");

//     println!("You guessed {user_guess}");
}
