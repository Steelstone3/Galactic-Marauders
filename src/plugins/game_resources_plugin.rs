use crate::resources::{
    combat_guild_reputation::CombatGuildReputation,
    exploration_guild_reputation::ExplorationGuildReputation, fleet_credits::FleetCredits,
    mine_ammunition::MineAmmunition, torpedo_ammunition::TorpedoAmmunition,
    trading_guild_reputation::TradingGuildReputation,
};
use bevy::prelude::{App, Plugin};

pub struct GameResourcesPlugin;

impl Plugin for GameResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FleetCredits(1000.0))
            .insert_resource(MineAmmunition(10.0))
            .insert_resource(TorpedoAmmunition(10.0))
            .insert_resource(CombatGuildReputation(1.0))
            .insert_resource(ExplorationGuildReputation(1.0))
            .insert_resource(TradingGuildReputation(1.0));
    }
}
