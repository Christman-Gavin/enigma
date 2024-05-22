use std::collections::HashMap;

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

struct Rotor {
    id: String, // i.e. I, II, III, IV, V...
    list: [char; 26],
}

type RotorSettings = [Rotor; 3];


type EnigmaRotors = {
    rotors
}



struct Enigma {
    plugboard: EnigmaPlugboard,
    rotors: [Rotor; 3],
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

impl Enigma {
    fn new(plugboard_settings: PlugboardSettings, rotors: RotorSettings) -> Self {
        let plugboard = get_plugboard(plugboard_settings);

        return Self { plugboard };
    }

    fn encode(&self, to_encode: &str) -> String {
        let mut return_string = "".to_string();

        let arred = to_encode.to_lowercase().chars();

        for char in arred {
            let plugboard_res = self.plugboard.get(&char);

            match plugboard_res {
                Some(found_plugboard_match) => return_string += &found_plugboard_match.to_string(),

                None => return_string += &char.to_string(),
            }
        }

        return "".to_string();
    }

    fn decode(&self, to_encode: &str) -> String {
        return "".to_string();
    }
}

fn main() {
    let plugboard_settings: PlugboardSettings =
        vec![('C', 'D'), ('R', 'T'), ('B', 'V'), ('X', 'P')];

    let enigma: Enigma = Enigma::new(plugboard_settings);

    let res = enigma.decode("Hello World");
}
