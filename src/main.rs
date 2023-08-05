use bevy::{app::AppExit, prelude::*, window::PresentMode};

const WINDOW_SIZE: f32 = 600.;

#[derive(Resource, Default, Clone)]
struct GameAssets {
	x: Handle<Image>,
	o: Handle<Image>,
}

#[derive(Resource, Default, Clone)]
struct Board([[Option<char>; 3]; 3]);

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum Turn {
	#[default]
	X,
	O,
}

impl Turn {
	fn char(self) -> char {
		let char = format!("{self:?}");
		char.chars().next().unwrap()
	}
	pub const fn not(self) -> Self {
		match self {
			Self::O => Self::X,
			Self::X => Self::O,
		}
	}
}

fn main() {
	App::new()
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			primary_window: Some(Window {
				title: "tic tac toe".into(),
				resolution: (WINDOW_SIZE, WINDOW_SIZE).into(),
				present_mode: PresentMode::AutoVsync,
				fit_canvas_to_parent: true,
				resizable: false,
				prevent_default_event_handling: false,
				..default()
			}),
			..default()
		}))
		.add_state::<Turn>()
		.init_resource::<GameAssets>()
		.init_resource::<Board>()
		.insert_resource(Msaa::Sample8)
		.add_systems(Startup, (setup_camera, draw_grid_system))
		.add_systems(Update, (click_system, check_victory_system))
		.run();
}

fn setup_camera(
	mut commands: Commands,
	mut game_assets: ResMut<GameAssets>,
	asset_server: Res<AssetServer>,
) {
	commands.spawn(Camera2dBundle::default());
	game_assets.x = asset_server.load("x.png");
	game_assets.o = asset_server.load("o.png");
}

fn draw_grid_system(mut commands: Commands) {
	let frac = WINDOW_SIZE / 6.;
	for i in &[-1, 1] {
		commands.spawn((SpriteBundle {
			transform: Transform {
				translation: Vec3::new(0., frac * *i as f32, 0.),
				scale: Vec3::new(WINDOW_SIZE, 10., 0.0),
				..default()
			},
			sprite: Sprite {
				color: Color::WHITE,
				..default()
			},
			..default()
		},));
	}
	for i in &[-1, 1] {
		commands.spawn((SpriteBundle {
			transform: Transform {
				translation: Vec3::new(frac * *i as f32, 0., 0.),
				scale: Vec3::new(10., WINDOW_SIZE, 0.0),
				..default()
			},
			sprite: Sprite {
				color: Color::WHITE,
				..default()
			},
			..default()
		},));
	}
}

fn click_system(
	commands: Commands,
	game_assets: Res<GameAssets>,
	windows: Query<&Window>,
	mouse_button_input: Res<Input<MouseButton>>,
	next_state: ResMut<NextState<Turn>>,
	current_state: Res<State<Turn>>,
	mut board: ResMut<Board>,
) {
	let asset = if current_state.get() == &Turn::O {
		&game_assets.o
	} else {
		&game_assets.x
	};
	let window = windows.get_single().unwrap();
	if let Some(position) = window.cursor_position() {
		if mouse_button_input.just_pressed(MouseButton::Left) {
			match position {
				pos if pos.x <= 200. && pos.y <= 200. => {
					//Top left
					spawn_piece(
						&mut board,
						current_state,
						next_state,
						commands,
						asset,
						Vec3::new(-200., 200., 0.),
						(0, 0),
					);
				}
				pos if pos.x <= 400. && pos.y <= 200. => {
					//Top mid
					spawn_piece(
						&mut board,
						current_state,
						next_state,
						commands,
						asset,
						Vec3::new(0., 200., 0.),
						(0, 1),
					);
				}
				pos if pos.x >= 400. && pos.y <= 200. => {
					//Top right
					spawn_piece(
						&mut board,
						current_state,
						next_state,
						commands,
						asset,
						Vec3::new(200., 200., 0.),
						(0, 2),
					);
				}
				pos if pos.x <= 200. && pos.y <= 400. => {
					//Mid left
					spawn_piece(
						&mut board,
						current_state,
						next_state,
						commands,
						asset,
						Vec3::new(-200., 0., 0.),
						(1, 0),
					);
				}
				pos if pos.x <= 400. && pos.y <= 400. => {
					//Center
					spawn_piece(
						&mut board,
						current_state,
						next_state,
						commands,
						asset,
						Vec3::new(0., 0., 0.),
						(1, 1),
					);
				}
				pos if pos.x >= 400. && pos.y <= 400. => {
					//Mid right
					spawn_piece(
						&mut board,
						current_state,
						next_state,
						commands,
						asset,
						Vec3::new(200., 0., 0.),
						(1, 2),
					);
				}
				pos if pos.x <= 200. && pos.y >= 400. => {
					//Bot left
					spawn_piece(
						&mut board,
						current_state,
						next_state,
						commands,
						asset,
						Vec3::new(-200., -200., 0.),
						(2, 0),
					);
				}
				pos if pos.x <= 400. && pos.y >= 400. => {
					//Bot mid
					spawn_piece(
						&mut board,
						current_state,
						next_state,
						commands,
						asset,
						Vec3::new(0., -200., 0.),
						(2, 1),
					);
				}
				pos if pos.x >= 400. && pos.y >= 400. => {
					//Bot right
					spawn_piece(
						&mut board,
						current_state,
						next_state,
						commands,
						asset,
						Vec3::new(200., -200., 0.),
						(2, 2),
					);
				}
				_ => {}
			};
		}
	}
}

fn spawn_piece(
	board: &mut ResMut<Board>,
	current_state: Res<State<Turn>>,
	mut next_state: ResMut<NextState<Turn>>,
	mut commands: Commands,
	asset: &Handle<Image>,
	pos: Vec3,
	index: (usize, usize),
) {
	if board.0[index.0][index.1].is_none() {
		board.0[index.0][index.1] = Some(current_state.get().char());
		next_state.set(current_state.not());
		commands.spawn(SpriteBundle {
			texture: asset.clone(),
			transform: Transform {
				translation: pos,
				scale: Vec3::new(0.75, 0.75, 0.0),
				..default()
			},
			..default()
		});
	}
}

fn check_victory_system(board: Res<Board>, mut ev_exit: EventWriter<AppExit>) {
	let winning_combinations = [
		[(0, 0), (0, 1), (0, 2)], // Rows
		[(1, 0), (1, 1), (1, 2)],
		[(2, 0), (2, 1), (2, 2)],
		[(0, 0), (1, 0), (2, 0)], // Columns
		[(0, 1), (1, 1), (2, 1)],
		[(0, 2), (1, 2), (2, 2)],
		[(0, 0), (1, 1), (2, 2)], // Diagonals
		[(0, 2), (1, 1), (2, 0)],
	];
	let board = board.0;

	let is_full = board.iter().flatten().all(Option::is_some);

	let mut won = false;
	for combination in winning_combinations.iter() {
		let (x1, y1) = combination[0];
		let (x2, y2) = combination[1];
		let (x3, y3) = combination[2];

		if let Some(player) = board[x1][y1] {
			if board[x2][y2] == Some(player) && board[x3][y3] == Some(player) {
				// println!("{player} won");
				ev_exit.send(AppExit);
				won = true;
				break;
			}
		}
	}
	if won == false && is_full {
		println!("nobody won");
		ev_exit.send(AppExit);
	}
}
