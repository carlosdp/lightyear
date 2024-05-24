//! This module contains the shared code between the client and the server.
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::utils::Duration;

use lightyear::prelude::*;
use lightyear::shared::config::Mode;

pub const FIXED_TIMESTEP_HZ: f64 = 64.0;

/// The [`SharedConfig`] must be shared between the `ClientConfig` and `ServerConfig`
pub fn shared_config(mode: Mode) -> SharedConfig {
    SharedConfig {
        // send an update every frame
        client_send_interval: Duration::default(),
        // send an update every 100ms
        server_send_interval: Duration::from_millis(100),
        tick: TickConfig {
            tick_duration: Duration::from_secs_f64(1.0 / FIXED_TIMESTEP_HZ),
        },
        mode,
    }
}

#[derive(Clone)]
pub struct SharedPlugin;

#[derive(Channel)]
pub struct Channel1;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Message1(pub usize);

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        // Register your protocol, which is shared between client and server
        app.add_message::<Message1>(ChannelDirection::Bidirectional);
        app.add_channel::<Channel1>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            ..default()
        });

        if app.is_plugin_added::<RenderPlugin>() {
            app.add_systems(Startup, init);
        }
    }
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}