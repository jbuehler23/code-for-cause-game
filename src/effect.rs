use bevy::asset::AssetLoader;
use bevy::{asset::Asset, reflect::Reflect};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::AssetLoadingError;

pub(super) fn plugin(app: &mut App) {
    app.init_asset_loader::<EffectLoader>()
        .init_asset::<Effect>();
}

#[derive(Asset, Reflect)]
pub struct Effect {
    icon: Handle<Image>,
    action: EffectAction,
}

#[derive(Reflect, Deserialize, Serialize)]
struct EffectAction {
    kind: ActionKind,
    value: u32,
}

#[derive(Reflect, Deserialize, Serialize)]
enum ActionKind {
    Damage,
    Heal,
}

#[derive(Default)]
struct EffectLoader;

#[derive(Deserialize, Serialize)]
pub struct EffectRon {
    icon: String,
    action: EffectAction,
}

impl AssetLoader for EffectLoader {
    type Asset = Effect;

    type Settings = ();

    type Error = AssetLoadingError;

    fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &Self::Settings,
        load_context: &mut bevy::asset::LoadContext,
    ) -> impl bevy::utils::ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes);
            let effect_ron: EffectRon = ron::de::from_bytes(&bytes)?;

            let icon = load_context.load::<Image>(&effect_ron.icon);

            Ok(Effect {
                icon,
                action: effect_ron.action,
            })
        })
    }
}
