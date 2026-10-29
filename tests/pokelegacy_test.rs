use bot_pokelegacy_rust::game::bot::BotInstance;
use bot_pokelegacy_rust::game::type_chart::get_type_weakness_multiplier;
use bot_pokelegacy_rust::game::types::*;
use serde_json::json;

#[test]
fn test_pokelegacy_weakness_chart() {
    assert_eq!(get_type_weakness_multiplier("water", "fire"), 2.0);
    assert_eq!(get_type_weakness_multiplier("fire", "grass"), 2.0);
    assert_eq!(get_type_weakness_multiplier("electric", "water"), 2.0);
    assert_eq!(get_type_weakness_multiplier("normal", "ghost"), 0.0);
}

#[test]
fn test_pokelegacy_team_counter_selection() {
    let cfg = AccountConfig {
        id: "leg-1".to_string(),
        name: "LegacyBot".to_string(),
        enabled: true,
        credentials: GameCredentials::default(),
        settings: GameSettings::default(),
    };
    let bot = BotInstance::new(cfg);

    let team = vec![
        Pokemon {
            id: "p1".to_string(),
            species_id: 6,
            name: "Charizard".to_string(),
            level: 50,
            types: vec!["fire".to_string(), "flying".to_string()],
            hp: 200,
            max_hp: 200,
            shiny: false,
        },
        Pokemon {
            id: "p2".to_string(),
            species_id: 9,
            name: "Blastoise".to_string(),
            level: 50,
            types: vec!["water".to_string()],
            hp: 220,
            max_hp: 220,
            shiny: false,
        },
    ];

    // Against Fire enemy, Blastoise should be chosen
    let chosen = bot.pick_best_pokemon_counter(&team, "fire");
    assert!(chosen.is_some());
    assert_eq!(chosen.unwrap().name, "Blastoise");

    // Against Grass enemy, Charizard should be chosen
    let chosen = bot.pick_best_pokemon_counter(&team, "grass");
    assert!(chosen.is_some());
    assert_eq!(chosen.unwrap().name, "Charizard");
}

#[test]
fn test_pokelegacy_save_envelope() {
    let st = PlayerState {
        username: "Ash".to_string(),
        level: 25,
        xp: 45000,
        gold: 12000,
        team: vec![],
        active_hunt: "route-1".to_string(),
    };
    let env = SaveEnvelope {
        state: st,
        client_timestamp: 1700000000,
    };
    let val = serde_json::to_value(&env).unwrap();
    assert_eq!(val["state"]["username"], "Ash");
    assert_eq!(val["state"]["level"], 25);
    assert_eq!(val["clientTimestamp"], 1700000000);
}
