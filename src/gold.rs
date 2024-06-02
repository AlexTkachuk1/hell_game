use bevy::prelude::*;

use crate::player::{GoldCount, Player};
use crate::state::GameState;

#[derive(Component)]
pub struct Gold;

pub struct GoldPlugin;

#[derive(Event)]
pub struct PlayerGoldCollisionEvent;

impl Plugin for GoldPlugin {
    fn build(&self, app: &mut App) {
      app.add_event::<PlayerGoldCollisionEvent>()
      .add_systems(
            Update,
            (
                  handle_player_gold_collision_events,
            ).run_if(in_state(GameState::InGame))
      );
    }
}

fn handle_player_gold_collision_events(
      mut player_query: Query<&mut GoldCount, With<Player>>,
      mut events: EventReader<PlayerGoldCollisionEvent>,
  ) {
      if player_query.is_empty() {
          return;
      }
  
      let mut gold = player_query.single_mut();
      for _ in events.read() {
            gold.0 += 1.;
      }
  }
  