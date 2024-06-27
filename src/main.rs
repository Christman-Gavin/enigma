use std::{char, collections::HashMap, time::Instant};

// Rotor    ABCDEFGHIJKLMNOPQRSTUVWXYZ 	    Date-Introduced 	Model Name & Number

// I 	    EKMFLGDQVZNTOWYHXUSPAIBRCJ 	    1930 	            Enigma I
// II 	    AJDKSIRUXBLHWTMCQGZNPYFVOE 	    1930 	            Enigma I
// III 	    BDFHJLCPRTXVZNYEIWGAKMUSQO 	    1930 	            Enigma I
// IV 	    ESOVPZJAYQUIRHXLNFTGKDCMWB 	    December 1938 	    M3 Army
// V 	    VZBRGITYUPSDNHLXAWMJQOFECK 	    December 1938 	    M3 Army
// VI 	    JPGVOUMFYQBENHZRDKASXLICTW 	    1939 	            M3 & M4 Naval (FEB 1942)
// VII 	    NZJHGRCXMYSWBOUFAIVLPEKQDT 	    1939 	            M3 & M4 Naval (FEB 1942)
// VIII 	FKQHTLXOCBJSPDZRAMEWNIUYGV 	    1939 	            M3 & M4 Naval (FEB 1942)

// UKW

// UKW-B    YRUHQSLDPXNGOKMIEBFZCWVJAT

// https://enigma.virtualcolossus.co.uk/technical.html

type CharList = [char; 26];

struct Rotor {
    char_list: CharList,
    notch: String,
    turnover: String,
    current_position: String,
}

impl Rotor {
    fn new(char_list: CharList, notch: char, turnover: char, init_position: char) -> Self {
        validate_char(notch);
        validate_char(turnover);
        validate_char(init_position);

        let times_to_shift = init_position as i32 - 39;

        let shifted_slice = shift_slice_x_times(char_list, times_to_shift);

        let rotor: Rotor = Rotor {
            char_list: shifted_slice,
            notch: notch.to_string().to_uppercase(),
            turnover: turnover.to_string().to_uppercase(),
            current_position: init_position.to_string().to_ascii_uppercase(),
        };

        return rotor;
    }

    // TODO: implement
    fn rotate() {}

    // TODO: implement
    fn get_rotor_response() {}
}

type Rotors = [Rotor; 3];
type UKW = CharList;

struct RotorMachine {
    rotors: Rotors,
    ukw: UKW,
}

// array of char-to-char relations to
type PlugboardSettings = Vec<(char, char)>;

type EnigmaPlugboard = HashMap<char, char>;

fn get_plugboard(plugboard_settings: PlugboardSettings) -> EnigmaPlugboard {
    let mut enigma_plugboard: EnigmaPlugboard = HashMap::new();

    for (i, j) in plugboard_settings {
        enigma_plugboard.insert(i, j);
        enigma_plugboard.insert(j, i);
    }

    return enigma_plugboard;
}

fn shift_slice_x_times(input: CharList, shifts: i32) -> CharList {
    let mut input_copy = input;

    let mut current_index: i32 = 0;

    for val in input {
        let new_index = (current_index + shifts) % 26;

        input_copy[new_index as usize] = val;

        current_index += 1
    }

    return input_copy;
}

fn plugboard_machine_map(input: char, plugboard: &EnigmaPlugboard) -> char {
    let plugboard_res = plugboard.get(&input);

    match plugboard_res {
        Some(found_plugboard_match) => *found_plugboard_match,

        None => input,
    }
}

// fn get_rotor_result() -> char {}

// This function is gonna suck fucking donkey-dick-balls
fn get_rotor_machine_response(char: char, mut rotor_machine: &RotorMachine) -> char {
    // first trip right-to-left

    let mut index = 3;

    while index > 0 {
        // logic

        let mut current_rotor: &Rotor = &rotor_machine.rotors[index as usize];

        index -= 1;
    }

    // UKW response

    // left-to-right

    return 'a';
}

fn validate_char(input: char) -> () {
    let allowed_chars = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    if !allowed_chars.contains(&input) {
        panic!(
            "char not allowed: '{}' Please refer to documentation to see list of allowed chars",
            input
        );
    }
}

fn validate(input: &str) {
    let allowed_chars = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    for (_, char) in input.chars().enumerate() {
        if !allowed_chars.contains(&char) {
            panic!(
                "char not allowed: '{}' Please refer to documentation to see list of allowed chars",
                char
            );
        }
    }
}

// TODO: expand this function
fn remove_uneeded(input: &str) -> String {
    let mut result = String::new();

    for (_, char) in input.chars().enumerate() {
        if char != ' ' {
            result += &char.to_string()
        }
    }

    return result;
}

struct Enigma {
    plugboard: EnigmaPlugboard,
    rotor_machine: RotorMachine,
}

impl Enigma {
    fn new(plugboard_settings: PlugboardSettings, rotors: Rotors, ukw: UKW) -> Self {
        let plugboard = get_plugboard(plugboard_settings);

        let rotor_machine = RotorMachine { rotors, ukw };

        return Self {
            plugboard,
            rotor_machine,
        };
    }

    fn cypher(self, to_encode: &str) -> String {
        let mut return_string = "".to_string();

        let trimmed = remove_uneeded(to_encode);

        let ascii_uppercase = trimmed.to_ascii_uppercase();

        validate(&ascii_uppercase);

        for char in ascii_uppercase.chars() {
            let first_plugboard_response = plugboard_machine_map(char, &self.plugboard);

            let rotor_response =
                get_rotor_machine_response(first_plugboard_response, &self.rotor_machine);

            let second_plugboard_response = plugboard_machine_map(rotor_response, &self.plugboard);

            return_string += &(second_plugboard_response.to_string())
        }

        return return_string;
    }
}

#[allow(non_snake_case)]
fn main() {
    let first_char_list: CharList = [
        'E', 'K', 'M', 'F', 'L', 'G', 'D', 'Q', 'V', 'Z', 'N', 'T', 'O', 'W', 'Y', 'H', 'X', 'U',
        'S', 'P', 'A', 'I', 'B', 'R', 'C', 'J',
    ];

    let second_char_list: CharList = [
        'A', 'J', 'D', 'K', 'S', 'I', 'R', 'U', 'X', 'B', 'L', 'H', 'W', 'T', 'M', 'C', 'Q', 'G',
        'Z', 'N', 'P', 'Y', 'F', 'V', 'O', 'E',
    ];

    let third_char_list: CharList = [
        'B', 'D', 'F', 'H', 'J', 'L', 'C', 'P', 'R', 'T', 'X', 'V', 'Z', 'N', 'Y', 'E', 'I', 'W',
        'G', 'A', 'K', 'M', 'U', 'S', 'Q', 'O',
    ];

    let rotor_I = Rotor::new(first_char_list, 'Y', 'Q', 'A');

    let rotor_II = Rotor::new(second_char_list, 'M', 'E', 'A');

    let rotor_III = Rotor::new(third_char_list, 'D', 'V', 'A');

    let ukw: UKW = [
        'Y', 'R', 'U', 'H', 'Q', 'S', 'L', 'D', 'P', 'X', 'N', 'G', 'O', 'K', 'M', 'I', 'E', 'B',
        'F', 'Z', 'C', 'W', 'V', 'J', 'A', 'T',
    ];

    let plugboard_settings: PlugboardSettings =
        vec![('C', 'D'), ('R', 'T'), ('B', 'V'), ('X', 'P')];

    let enigma: Enigma = Enigma::new(plugboard_settings, [rotor_I, rotor_II, rotor_III], ukw);

    let res = enigma.cypher("Hello World");

    println!("{res}");
}
