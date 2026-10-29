#[derive(Debug, Clone)]
pub struct AreaCandidate {
    pub id: &'static str,
    pub min_level: u32,
    pub max_level: u32,
    pub base_exp: f64,
    pub base_gold: f64,
}

pub static HUNTS_CATALOG: &[AreaCandidate] = &[
        AreaCandidate { id: "kanto_campos_de_pallet", min_level: 1, max_level: 18, base_exp: 110.0, base_gold: 55.0 },
        AreaCandidate { id: "kanto_floresta_viridian", min_level: 18, max_level: 38, base_exp: 290.0, base_gold: 155.0 },
        AreaCandidate { id: "kanto_monte_lua", min_level: 38, max_level: 65, base_exp: 740.0, base_gold: 420.0 },
        AreaCandidate { id: "kanto_caverna_cerulean", min_level: 65, max_level: 100, base_exp: 1900.0, base_gold: 1150.0 },
        AreaCandidate { id: "kanto_tunel_rocha", min_level: 100, max_level: 150, base_exp: 4700.0, base_gold: 3000.0 },
        AreaCandidate { id: "kanto_ilha_cinnabar", min_level: 150, max_level: 230, base_exp: 11800.0, base_gold: 7800.0 },
];

#[derive(Clone, Default)]
pub struct Catalog;

impl Catalog {
    pub fn select_best_hunt(&self, level: u32) -> &'static AreaCandidate {
        for area in HUNTS_CATALOG {
            if level >= area.min_level && level <= area.max_level {
                return area;
            }
        }
        &HUNTS_CATALOG[HUNTS_CATALOG.len() - 1]
    }
}
