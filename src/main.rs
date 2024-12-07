use std::collections::HashSet;

fn main() {
    let text1 = String::from("Hola Mundo");
    let text2 = String::from("Hola");
    print_result(&text1, has_all_unique_characters_double_bucle(&text1));
    print_result(&text2, has_all_unique_characters_double_bucle(&text2));
    print_result(&text1, has_all_unique_characters_with_sorting(&text1));
    print_result(&text2, has_all_unique_characters_with_sorting(&text2));
    print_result(&text1, has_all_unique_characters_ascii_buffer(&text1));
    print_result(&text2, has_all_unique_characters_ascii_buffer(&text2));
    print_result(&text1, has_all_unique_characters_with_map(&text1));
    print_result(&text2, has_all_unique_characters_with_map(&text2));
}

fn has_all_unique_characters_with_map(text: &String) -> bool {
    if text.len() == 0 || text.len() == 1 {
        return true;
    }

    let chars: Vec<char> = text.chars().collect();
    let mut characters: HashSet<char> = HashSet::new();

    for c in chars {
        if characters.contains(&c) {
            return false
        }
        characters.insert(c);
    }
    true
}

fn has_all_unique_characters_ascii_buffer(text: &String) -> bool {
    if text.len() == 0 || text.len() == 1 {
        return true;
    }

    let mut buffer:  [bool; 128] = [false; 128];
    let chars: Vec<char> = text.chars().collect();
    for character in chars {
        let buffer_index = character as u8;
        if buffer[buffer_index as usize] {
            return false;
        }
        buffer[buffer_index as usize] = true;
    }
    true
}

fn has_all_unique_characters_with_sorting(text: &String) -> bool {
    if text.len() == 0 || text.len() == 1 {
        return true;
    }

    let mut chars :Vec<char> = text.chars().collect();
    chars.sort();
    for i in 0..chars.len() - 1 {
        if chars[i] == chars[i + 1] {
            return false;
        }
    }
    true
}

fn has_all_unique_characters_double_bucle(text: &String) -> bool {
    
    if text.len() == 0 || text.len() == 1 {
        return true;
    }

    let chars : Vec<char> = text.chars().collect();

    for i in 0..chars.len() {
        for j in i + 1..chars.len() {
            if chars[i] == chars[j] {
                return false;
            }
        }
    }
    true
}


fn print_result(text: &String, result: bool) {
    if result {
        println!("The text '{}' hasn't duplicated chars", text);
    } else {
        println!("The text '{}' has duplicated chars", text);
    }
}