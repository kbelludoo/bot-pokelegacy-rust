use crate::engine::circuit_breaker::{CircuitBreaker, CircuitBreakerConfig};
use crate::engine::stamina::{StaminaConfig, StaminaController};
use crate::game::api::LegacyClient;
use crate::game::type_chart::get_type_weakness_multiplier;
use crate::game::types::{AccountConfig, PlayerState, Pokemon};
use crate::subsystem::dailies::DailyManager;
use crate::subsystem::inventory::InventoryManager;
use crate::subsystem::potions::PotionManager;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use tracing::{debug, error, info, warn};

pub struct BotInstance {
    pub config: AccountConfig,
    pub circuit_breaker: Arc<CircuitBreaker>,
    pub stamina: Arc<StaminaController>,
    pub inventory: Arc<InventoryManager>,
    pub dailies: Arc<DailyManager>,
    pub potions: Arc<PotionManager>,
    pub api: Arc<LegacyClient>,

    pub state: Arc<Mutex<PlayerState>>,
    pub kills: AtomicU64,
    pub gold: AtomicU64,
    pub exp: AtomicU64,
    pub current_level: AtomicU32,
    pub running: AtomicBool,
}

impl BotInstance {
    pub fn new(config: AccountConfig) -> Self {
        let cb_cfg = CircuitBreakerConfig::default();
        let cb = Arc::new(CircuitBreaker::new(&config.id, cb_cfg));
        let stam = Arc::new(StaminaController::new(StaminaConfig::default()));
        let api = Arc::new(LegacyClient::new(&config));

        Self {
            config,
            circuit_breaker: cb,
            stamina: stam,
            inventory: Arc::new(InventoryManager::default()),
            dailies: Arc::new(DailyManager::default()),
            potions: Arc::new(PotionManager::default()),
            api,
            state: Arc::new(Mutex::new(PlayerState::default())),
            kills: AtomicU64::new(0),
            gold: AtomicU64::new(0),
            exp: AtomicU64::new(0),
            current_level: AtomicU32::new(1),
            running: AtomicBool::new(false),
        }
    }

    pub async fn init(&self) -> Result<(), String> {
        info!("Autenticando sessão no PokeLegacy...");
        let me = self.api.fetch_me().await?;
        if let Some(st) = me.state {
            self.current_level.store(st.level, Ordering::Relaxed);
            self.gold.store(st.gold as u64, Ordering::Relaxed);
            self.exp.store(st.xp, Ordering::Relaxed);
            *self.state.lock().await = st;
        }
        Ok(())
    }

    pub fn pick_best_pokemon_counter<'a>(&self, team: &'a [Pokemon], enemy_type: &str) -> Option<&'a Pokemon> {
        let mut best: Option<&'a Pokemon> = None;
        let mut best_mult = 0.0;

        for p in team {
            if p.hp == 0 {
                continue;
            }
            for t in &p.types {
                let mult = get_type_weakness_multiplier(t, enemy_type);
                if mult > best_mult {
                    best_mult = mult;
                    best = Some(p);
                }
            }
        }

        best
    }

    pub async fn perform_hunt_cycle(&self) {
        let mut st = self.state.lock().await;

        // Simulação de caça: abates acumulados e ganho de xp/ouro
        st.xp += 350;
        st.gold += 120;
        self.exp.store(st.xp, Ordering::Relaxed);
        self.gold.store(st.gold as u64, Ordering::Relaxed);
        self.kills.fetch_add(1, Ordering::Relaxed);

        if self.kills.load(Ordering::Relaxed) % 10 == 0 {
            info!("⚔️ PokeLegacy: 10 novos abates confirmados na rota '{}'!", st.active_hunt);
        }
    }

    pub async fn run_loop(self: Arc<Self>) {
        self.running.store(true, Ordering::SeqCst);
        info!("🚀 Bot [{}] (PokeLegacy Online) iniciado com sucesso.", self.config.name);

        let _ = self.init().await;

        let mut hunt_timer = tokio::time::interval(Duration::from_secs(3));
        let mut save_timer = tokio::time::interval(Duration::from_secs(30));
        let mut daily_timer = tokio::time::interval(Duration::from_secs(300));

        while self.running.load(Ordering::Relaxed) {
            tokio::select! {
                _ = hunt_timer.tick() => {
                    if self.config.settings.auto_hunt {
                        self.perform_hunt_cycle().await;
                    }
                }
                _ = save_timer.tick() => {
                    let st = self.state.lock().await.clone();
                    let _ = self.api.push_save(&st).await;
                    if self.config.settings.auto_sell {
                        let _ = self.api.sell_trash().await;
                    }
                }
                _ = daily_timer.tick() => {
                    if self.config.settings.auto_daily {
                        let _ = self.api.claim_daily_reward().await;
                    }
                }
            }
        }
    }
}
