use soroban_sdk::{contracttype, Address, Bytes, Env};

/// Badge tier awarded based on cumulative score thresholds.
/// Scores are in basis points (0–10 000).
///
/// Thresholds:
///   Bronze  ≥ 4 000
///   Silver  ≥ 6 000
///   Gold    ≥ 8 000
///   Platinum ≥ 9 500
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BadgeLevel {
    None,
    Bronze,
    Silver,
    Gold,
    Platinum,
}

impl BadgeLevel {
    pub fn from_score(score: i32) -> Self {
        match score {
            s if s >= 9_500 => BadgeLevel::Platinum,
            s if s >= 8_000 => BadgeLevel::Gold,
            s if s >= 6_000 => BadgeLevel::Silver,
            s if s >= 4_000 => BadgeLevel::Bronze,
            _ => BadgeLevel::None,
        }
    }
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Profile {
    pub address: Address,
    pub client_score: i32,
    pub client_points: i32,
    pub client_jobs: u32,
    pub client_badge: BadgeLevel,
    pub freelancer_score: i32,
    pub freelancer_points: i32,
    pub freelancer_jobs: u32,
    pub freelancer_badge: BadgeLevel,
    pub metadata_hash: Option<Bytes>,
}

impl Profile {
    pub fn new(_env: &Env, address: Address) -> Self {
        Self {
            address,
            client_score: 5000,
            client_points: 0,
            client_jobs: 0,
            client_badge: BadgeLevel::Bronze, // 5000 ≥ 4000
            freelancer_score: 5000,
            freelancer_points: 0,
            freelancer_jobs: 0,
            freelancer_badge: BadgeLevel::Bronze,
            metadata_hash: None,
        }
    }

    /// Recompute badge levels from current scores.
    pub fn refresh_badges(&mut self) {
        self.client_badge = BadgeLevel::from_score(self.client_score);
        self.freelancer_badge = BadgeLevel::from_score(self.freelancer_score);
    }

    pub fn default(_env: Env) -> Self {
        // This is tricky because we need an address.
        // We'll leave it to the caller to provide an address.
        panic!("Profile needs an address; use new(env, address)")
    }
}
