pub fn get_type_weakness_multiplier(atk_type: &str, def_type: &str) -> f64 {
    let atk = atk_type.to_lowercase();
    let def = def_type.to_lowercase();

    match (atk.as_str(), def.as_str()) {
        ("water", "fire") | ("water", "ground") | ("water", "rock") => 2.0,
        ("fire", "grass") | ("fire", "ice") | ("fire", "bug") | ("fire", "steel") => 2.0,
        ("grass", "water") | ("grass", "ground") | ("grass", "rock") => 2.0,
        ("electric", "water") | ("electric", "flying") => 2.0,
        ("ice", "grass") | ("ice", "ground") | ("ice", "flying") | ("ice", "dragon") => 2.0,
        ("fighting", "normal") | ("fighting", "ice") | ("fighting", "rock") | ("fighting", "dark") | ("fighting", "steel") => 2.0,
        ("poison", "grass") | ("poison", "fairy") => 2.0,
        ("ground", "fire") | ("ground", "electric") | ("ground", "poison") | ("ground", "rock") | ("ground", "steel") => 2.0,
        ("flying", "grass") | ("flying", "fighting") | ("flying", "bug") => 2.0,
        ("psychic", "fighting") | ("psychic", "poison") => 2.0,
        ("bug", "grass") | ("bug", "psychic") | ("bug", "dark") => 2.0,
        ("rock", "fire") | ("rock", "ice") | ("rock", "flying") | ("rock", "bug") => 2.0,
        ("ghost", "psychic") | ("ghost", "ghost") => 2.0,
        ("dragon", "dragon") => 2.0,
        ("steel", "ice") | ("steel", "rock") | ("steel", "fairy") => 2.0,
        ("dark", "psychic") | ("dark", "ghost") => 2.0,
        ("fairy", "fighting") | ("fairy", "dragon") | ("fairy", "dark") => 2.0,

        // Imunidades
        ("normal", "ghost") | ("ghost", "normal") | ("electric", "ground") | ("ground", "flying") => 0.0,

        _ => 1.0,
    }
}
