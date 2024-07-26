use std::{
    char,
    collections::HashMap,
    time::{Duration, Instant},
};

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

#[derive(Debug, Clone, Copy)]
struct Rotor {
    right_to_left_char_list: CharList,
    left_to_right_char_list: CharList,
    notch: char,
    turnover: char,
    current_position: char,
}

impl Rotor {
    fn new(char_list: CharList, notch: char, turnover: char, init_position: char) -> Self {
        validate_char(notch);
        validate_char(turnover);
        validate_char(init_position);

        let times_to_shift = init_position as i32 - 39;

        let shifted_right_to_left_slice = shift_slice_x_times(char_list, times_to_shift);

        let mut left_to_right_slice: CharList = [
            '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!',
            '!', '!', '!', '!', '!', '!', '!', '!', '!',
        ];

        for (usize, char) in shifted_right_to_left_slice.iter().enumerate() {
            let index = (*char as u8 - 65) as usize;

            left_to_right_slice[index] = (usize as u8 + 65) as char;
        }

        let rotor: Rotor = Rotor {
            right_to_left_char_list: shifted_right_to_left_slice,
            left_to_right_char_list: left_to_right_slice,
            notch,
            turnover,
            current_position: init_position,
        };

        return rotor;
    }

    // returns true if the next rotor needs to rotate
    pub fn rotate(&mut self) -> bool {
        self.right_to_left_char_list = shift_slice_x_times(self.right_to_left_char_list, 1);

        let mut left_to_right_slice: CharList = [
            '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!', '!',
            '!', '!', '!', '!', '!', '!', '!', '!', '!',
        ];

        for (usize, char) in self.right_to_left_char_list.iter().enumerate() {
            let index = (*char as u8 - 65) as usize;

            left_to_right_slice[index] = (usize as u8 + 65) as char;
        }

        self.left_to_right_char_list = left_to_right_slice;

        self.current_position = increase_position(self.current_position);

        self.notch = decrease_position(self.current_position);

        if self.current_position as u8 >= self.turnover as u8 {
            return true;
        } else {
            return false;
        }
    }

    pub fn get_right_to_left_rotor_response(&self, char: char) -> char {
        self.right_to_left_char_list[(char as u8 - 65) as usize]
    }

    pub fn get_left_to_right_rotor_response(&self, char: char) -> char {
        self.left_to_right_char_list[(char as u8 - 65) as usize]
    }
}

type Rotors = [Rotor; 3];
type UKW = CharList;

#[derive(Debug, Clone, Copy)]
struct RotorMachine {
    rotors: Rotors,
    ukw: UKW,
}

// array of char-to-char relations to turn into a hashmap
type PlugboardSettings = Vec<(char, char)>;

type EnigmaPlugboard = HashMap<char, char>;

fn increase_position(current_position: char) -> char {
    ((current_position as u8 - 63) % 26 + 64) as char
}

fn decrease_position(current_position: char) -> char {
    let mut f = current_position as u8 - 1;

    if f < 65 {
        f += 26
    }

    f as char
}

fn get_plugboard(plugboard_settings: Option<PlugboardSettings>) -> Option<EnigmaPlugboard> {
    match plugboard_settings {
        Some(plugboard_settings) => {
            let mut enigma_plugboard: EnigmaPlugboard = HashMap::new();

            for (char_1, char_2) in plugboard_settings {
                enigma_plugboard.insert(char_1, char_2);
                enigma_plugboard.insert(char_2, char_1);
            }

            return Some(enigma_plugboard);
        }
        None => None,
    }
}

fn shift_slice_x_times(mut input: CharList, shifts: i32) -> CharList {
    let mut current_index: i32 = 0;

    for val in input {
        let new_index = (current_index + shifts) % 26;

        input[new_index as usize] = val;

        current_index += 1
    }

    return input;
}

fn plugboard_machine_map(input: char, plugboard: &Option<EnigmaPlugboard>) -> char {
    match plugboard {
        Some(plugboard) => {
            let plugboard_res = plugboard.get(&input);

            match plugboard_res {
                Some(found_plugboard_match) => *found_plugboard_match,

                None => input,
            }
        }
        None => input,
    }
}

pub struct RotorMachineResponse {
    char: char,
    rotor_machine: RotorMachine,
}

fn get_rotor_machine_response(
    char: char,
    rotor_machine: &mut RotorMachine,
) -> RotorMachineResponse {
    let mut current_response: char;

    // When a key is pressed, the first rotor rotates
    let need_to_rotate_next_rotor = rotor_machine.rotors[2].rotate();

    current_response = rotor_machine.rotors[2].get_right_to_left_rotor_response(char);

    // Rotate the next rotor if we've reached the turnover position
    if need_to_rotate_next_rotor {
        let need_to_rotate_next_rotor = rotor_machine.rotors[1].rotate();

        if need_to_rotate_next_rotor {
            let _ = rotor_machine.rotors[0].rotate();
        }
    }

    current_response = rotor_machine.rotors[1].get_right_to_left_rotor_response(current_response);

    current_response = rotor_machine.rotors[0].get_right_to_left_rotor_response(current_response);

    // message enters UKW
    let ukw_response = rotor_machine.ukw[(current_response as u8 - 65) as usize];

    // Message travels back through the rotors to the lightboard
    current_response = rotor_machine.rotors[0].get_left_to_right_rotor_response(ukw_response);

    current_response = rotor_machine.rotors[1].get_left_to_right_rotor_response(current_response);

    current_response = rotor_machine.rotors[2].get_left_to_right_rotor_response(current_response);

    return RotorMachineResponse {
        rotor_machine: rotor_machine.clone(),
        char: current_response,
    };
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

fn remove_uneeded(input: &str) -> String {
    let mut result = String::new();

    let not_allowed_chars = vec![
        '!', ',', '?', ';', ':', '/', '=', '+', '_', '-', ' ', '.', '\'',
    ];

    for (_, char) in input.chars().enumerate() {
        if !not_allowed_chars.contains(&char) {
            result += &char.to_string()
        }
    }

    return result;
}

struct Enigma {
    plugboard: Option<EnigmaPlugboard>,
    rotor_machine: RotorMachine,
}

impl Enigma {
    fn new(plugboard_settings: Option<PlugboardSettings>, rotors: Rotors, ukw: UKW) -> Self {
        let plugboard = get_plugboard(plugboard_settings);

        let rotor_machine = RotorMachine { rotors, ukw };

        return Self {
            plugboard,
            rotor_machine,
        };
    }

    fn cypher(&mut self, to_encode: &str) -> String {
        let mut return_string = "".to_string();

        let trimmed = remove_uneeded(to_encode);

        let ascii_uppercase = trimmed.to_ascii_uppercase();

        validate(&ascii_uppercase);

        for char in ascii_uppercase.chars() {
            let first_plugboard_response = plugboard_machine_map(char, &self.plugboard);

            let rotor_machine_response: RotorMachineResponse =
                get_rotor_machine_response(first_plugboard_response, &mut self.rotor_machine);

            self.rotor_machine = rotor_machine_response.rotor_machine;

            let second_plugboard_response =
                plugboard_machine_map(rotor_machine_response.char, &self.plugboard);

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

    #[allow(non_snake_case)]
    let rotor_I = Rotor::new(first_char_list, 'Y', 'Q', 'A');

    #[allow(non_snake_case)]
    let rotor_II = Rotor::new(second_char_list, 'M', 'E', 'A');

    #[allow(non_snake_case)]
    let rotor_III = Rotor::new(third_char_list, 'D', 'V', 'A');

    let ukw: UKW = [
        'Y', 'R', 'U', 'H', 'Q', 'S', 'L', 'D', 'P', 'X', 'N', 'G', 'O', 'K', 'M', 'I', 'E', 'B',
        'F', 'Z', 'C', 'W', 'V', 'J', 'A', 'T',
    ];

    let plugboard_settings_1: PlugboardSettings =
        vec![('C', 'D'), ('R', 'T'), ('B', 'V'), ('X', 'P')];

    let mut enigma1: Enigma = Enigma::new(
        Some(plugboard_settings_1),
        [rotor_I, rotor_II, rotor_III],
        ukw,
    );

    // "To be, or not to be, that is the question. Wether 'tis nobler in heart to suffer the slings and arrows of woe, or to take arms against a see of outrageous fortune. To live, to die, to live again."
    let cyphered = enigma1.cypher("Hello World!");

    let plugboard_settings_2: PlugboardSettings =
        vec![('C', 'D'), ('R', 'T'), ('B', 'V'), ('X', 'P')];

    let mut enigma2: Enigma = Enigma::new(
        Some(plugboard_settings_2),
        [rotor_I, rotor_II, rotor_III],
        ukw,
    );

    let decyphered = enigma2.cypher(&cyphered);

    println!("cyphered: {cyphered}");

    println!("decyphcered: {decyphered}");
}
