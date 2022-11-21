use bevy::{prelude::*, ecs::{system::Command, query}, reflect::erased_serde::__private::serde::__private::de, utils::tracing::instrument::WithSubscriber};
use crate::ScoreResource;
use crate::consts::*;
#[derive(Component)]
struct ColorText;

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("main menu enter");
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(50.0),
                top: Val::Px(100.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()    
    })
    // spawn text child
    .with_children(
        |parent| {
            parent.spawn((
                TextBundle::from_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    "",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 50.0,
                        color: Color::WHITE,
                    },
                ) // Set the alignment of the Text
                .with_text_alignment(TextAlignment::TOP_CENTER)
                // Set the style of the TextBundle itself.
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(10.0),
                        top: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                }),
                TimeText,
            ))
        ;}
    )
    ;

    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(50.0),
                top: Val::Px(500.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()    
    })
    // spawn text child
    .with_children(
        |parent| {
            parent.spawn((
                TextBundle::from_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    "",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 50.0,
                        color: Color::WHITE,
                    },
                ) // Set the alignment of the Text
                .with_text_alignment(TextAlignment::CENTER_LEFT)
                // Set the style of the TextBundle itself.
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(10.0),
                        top: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                }),
                ScoreText,
            ))
        ;}
    )
    ;
}

#[derive(Component)]
struct TimeText;

fn update_time_text(
    time: Res<Time>, 
    mut query: Query<(&mut Text, &TimeText)>,
) {
        // Song starts 3 seconds after real time
        let secs = time.elapsed_seconds_f64();

        // Don't do anything before the song starts
        if secs < 0. {
            return;
        }
    
        for (mut text, _marker) in query.iter_mut() {
            text.sections[0].value = format!("Time: {:.2}", secs);
            text.sections[0].style.color = Color::Rgba {
                red: (1.25 * secs as f32).sin() / 2.0 + 0.5,
                green: (0.75 * secs as f32).sin() / 2.0 + 0.5,
                blue: (0.50 * secs as f32).sin() / 2.0 + 0.5,
                alpha: 1.0,
            };
        }
}

#[derive(Component)]
struct ScoreText;

fn update_score_text(mut score: ResMut<ScoreResource>, mut query: Query<(&mut Text, &ScoreText)>) {
    for (mut text, _marker) in query.iter_mut() {
        text.sections[0].value = format!(
            "Score: {}.\nCorrects: {}.\nFails: {}\n",
            score.score(),
            score.corrects(),
            score.fails()
        );
    }
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(
            SystemSet::on_enter(AppState::Game)
            .with_system(setup_ui)
        )
        .add_system_set(
            SystemSet::on_update(AppState::Game)
            .with_system(update_time_text)
            .with_system(update_score_text)
        );
        // app.add_startup_system(setup_ui)
        // .add_system(update_time_text)
        // .add_system(update_score_text);
    }
}