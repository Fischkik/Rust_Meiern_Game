use rand;
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

#[derive(Debug)]
struct Spieler {
    namen: String,
    punktzahl: i32,
    ist_bot: bool,
    verhaltens_typ: char,
}

fn create_game() {
    let valid_behavior: [char; 4] = ['v', 'm', 'n', 'r'];
    let mut results = vec![];
    let mut player_count_str: String = String::new();
    let mut player_count: u32;
    let mut cpu_count_str: String = String::new();
    let mut cpu_count: u32;

    loop {
        player_count_str.clear();
        println!("Enter the number of Players");
        io::stdin()
            .read_line(&mut player_count_str)
            .expect("Failed procesing user input for Player count");
        player_count = match player_count_str.trim().parse() {
            Ok(number_player) => number_player,
            Err(_) => {
                println!("Failed to convert the user input into an integar");
                continue;
            }
        };

        cpu_count_str.clear();
        println!("Enter the number of CPU Players");
        io::stdin()
            .read_line(&mut cpu_count_str)
            .expect("Failed processing user input for Cpu count");
        cpu_count = match cpu_count_str.trim().parse() {
            Ok(number_cpu) => number_cpu,
            Err(_) => {
                println!("Failed to convert the user intpu into an integar");
                continue;
            }
        };

        if cpu_count > player_count - 1 {
            println!("cpu count to high");
            continue;
        }
        break;
    }
    for x in 0..cpu_count {
        let cpu_name: String = format!("payer{}", (x + 1));
        let behavior_index: usize = rand::random_range(0..(valid_behavior.len() - 1));
        let behavior: char = *valid_behavior
            .get(behavior_index)
            .expect("Failed during reading behavior");
        let ist_bot: bool = true;
        let cpu = create_player(cpu_name, behavior, ist_bot);
        results.push(cpu);
    }

    for i in 0..(player_count - cpu_count) {
        // --- Read name ---
        println!("Enter name:");
        let mut str_name = String::new();
        io::stdin()
            .read_line(&mut str_name)
            .expect("Failed processing user input for Player name");
        let name = str_name.trim().to_string();

        // --- Read behavior ---
        let verhaltens_typ: char = 'r';

        // --- Read bool ---
        let ist_bot: bool = false;

        // --- Call the function ---
        let player = create_player(name, verhaltens_typ, ist_bot);

        // --- Append result to list ---
        results.push(player);
    }
    for i in &results {
        println!("{:?}", i);
    }

    play_game(results);
}
//
fn create_player(name: String, verhaltens_typ: char, ist_bot: bool) -> Spieler {
    let erstellter_spieler = Spieler {
        namen: String::from(name),
        punktzahl: 0,
        ist_bot: ist_bot,
        verhaltens_typ: verhaltens_typ,
    };
    return erstellter_spieler;
}
//
fn play_game(spieler_liste: Vec<Spieler>) {
    let moegliche_wuerfe_array: [u8; 21] = [
        31, 32, 41, 42, 43, 51, 52, 53, 54, 61, 62, 63, 64, 65, 11, 22, 33, 44, 55, 66, 21,
    ];
    println!("Game Start");
    loop {
        for sp in &spieler_liste {
            println!("{}'s turn", sp.namen);
            println!("roling Dice");
            let wurf: u8 = wuerfel_wurf(&moegliche_wuerfe_array);
            println!("rolled {}", wurf);
        }
        break;
    }
}

// fn fninish_game(spieler_liste: &mut Vec<Spieler>) {}
//
// fn anzweifeln(wurf: &u8, ansage: &u8) -> bool {}
//
// fn ansagen(moegliche_werte: &[u8; 21]) -> u8 {}
//
fn wuerfel_wurf(moegliche_werte: &[u8; 21]) -> u8 {
    let wert_index: usize = rand::random_range(0..21);
    let wurf_wert: u8 = *moegliche_werte
        .get(wert_index)
        .expect("Failed during wuerfel_werfen");
    return wurf_wert;
}

//
// fn create_wuerfel(augenzahl: Option<u16>) -> Vec<u16> {
//     let augenzahl: u16 = augenzahl.unwrap_or(6);
//     let mut vector_augenzahlen: Vec<u16> = Vec::new();
//     for i in 0..augenzahl {
//         vector_augenzahlen.push(i);
//     }
//     return vector_augenzahlen;
// }

fn main() {
    // struct Wuerfel {
    //     augenzahl: Vec<u16>,
    // }
    //
    // let moegliche_wuerfe_array: [u8; 21] = [
    //     31, 32, 41, 42, 43, 51, 52, 53, 54, 61, 62, 63, 64, 65, 11, 22, 33, 44, 55, 66, 21,
    // ];
    //
    // // let wuerfel1_augenzahl: Vec<u16> = create_wuerfel(None);
    // let wuerfel2_augenzahl: Vec<u16> = create_wuerfel(None);
    //
    // let wuerfel1: Wuerfel = Wuerfel {
    //     augenzahl: wuerfel1_augenzahl,
    // };
    // let wuerfel2: Wuerfel = Wuerfel {
    //     augenzahl: wuerfel2_augenzahl,
    // };
    //
    // println!("{}", "-".repeat(20));
    println!("If you wich to start the game press enter");
    // this is means just take any input and do nothing with it
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
