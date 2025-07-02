use spacetimedb::Identity;
use spacetimedb::SpacetimeType;

#[spacetimedb::table(name = user)]
#[derive(Clone, Debug)]
pub struct SpaceTimeUser {
    #[primary_key]
    pub id: Identity,
    pub principal: String,
    pub display_name: Option<String>,
    pub pump_dump: PumpAndDump,
}

impl Default for SpaceTimeUser {
    fn default() -> Self {
        Self {
            id: Default::default(),
            principal: Default::default(),
            display_name: Default::default(),
            pump_dump: PumpAndDump {
                cents: 0,
                total_pumps: 0,
                total_dumps: 0,
            },
        }
    }
}

#[derive(Clone, Debug, SpacetimeType)]
pub struct PumpAndDump {
    cents: u128,
    total_pumps: u64,
    total_dumps: u64,
}

#[spacetimedb::table(name = liquidity_pools)]
pub struct LiquidityPool {
    #[primary_key]
    pub principal: String,
    pub amount: u128,
}

pub struct CentsToken {
    pub net_airdrop: u128,
    pub balance: u128,
    pub net_earning: u128,
}
