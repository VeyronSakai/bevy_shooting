use bevy::prelude::*;

pub struct GameUiPlugin;

#[derive(Component)]
pub struct Score {
    pub value: i32,
}

impl Score {
    fn new() -> Score {
        Score { value: 0 }
    }
}

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_stage("setup_ui", SystemStage::single(setup))
            .add_system(update_scrore_system);
    }
}

fn update_scrore_system(mut score_query: Query<(&Score, &mut Text)>) {
    let (score, mut text) = score_query.single_mut();

    text.sections[0].value = score.value.to_string();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: 0.to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.5, 0.5, 1.0),
                    },
                }],
                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Score::new());
}
