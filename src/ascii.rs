pub fn has_non_ascii(s: &str) -> bool {
    s.contains('á')
        || s.contains('é')
        || s.contains('í')
        || s.contains('ó')
        || s.contains('ú')
        || s.contains('ü')
}

pub fn asciify(c: char) -> char {
    match c {
        'á' => 'a',
        'é' => 'e',
        'í' => 'i',
        'ó' => 'o',
        'ú' | 'ü' => 'u',
        default => default,
    }
}

pub fn asciify_str(s: &str) -> String {
    s.to_owned().chars().map(|c| asciify(c)).collect()
}

/// Checks if a is similar to b
pub fn compare_chars(a: char, b: char) -> bool {
    a == asciify(b) || a == b
}