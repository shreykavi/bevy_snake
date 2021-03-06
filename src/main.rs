use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy::{core::FixedTimestep};

struct WinSize {
    w: f32,
    h: f32,
}

struct Direction {d: Movement}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

struct Snake {
    points: Vec<Point>,
}

#[derive(Debug, PartialEq, Eq)]
enum Movement {
    // Variants
    Up, Down, Left, Right
}

fn keyboard_control(    
    keyboard_input: Res<Input<KeyCode>>,
    mut direction: ResMut<Direction>
){
    if keyboard_input.pressed(KeyCode::Left) && direction.d != Movement::Right {
        direction.d = Movement::Left;
    } else if keyboard_input.pressed(KeyCode::Right) && direction.d != Movement::Left {
        direction.d = Movement::Right;
    } else if keyboard_input.pressed(KeyCode::Up) && direction.d != Movement::Down {
        direction.d = Movement::Up;
    } else if keyboard_input.pressed(KeyCode::Down) && direction.d != Movement::Up {
        direction.d = Movement::Down;
    };
}

fn snake_movement(
    mut commands: Commands,
    mut snake: ResMut<Snake>,
    direction: Res<Direction>
){
    // Removes tail and adds a new head
    //TODO: dont remove on consumed food
    snake.points.remove(0); // remove tail
    let mut head = Point{ x: snake.points.last().expect("").x, y: snake.points.last().expect("").y};

    // Increment new head based on direction
    match direction.d {
        Movement::Up => head.y += 1,
        Movement::Down => head.y -= 1,
        Movement::Right => head.x += 1,
        Movement::Left => head.x -= 1,
    }
    snake.points.push(Point{ x: head.x, y: head.y});
}

fn draw_system(
    mut commands: Commands, 
    mut snake: ResMut<Snake>
){
    //TODO: despawn and respawn the snake 
    println!("{:?}", snake.points);
}

fn food_system(
    mut commands: Commands, 
){
    //TODO: spawn food randomly
}

fn score_system(
    mut commands: Commands, 
    mut snake: ResMut<Snake>
){
    //TODO: show tally of score based on size of snake
}

fn open_menu_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    win_size: Res<WinSize>,
) {
    // TODO: Show show + a reset button for end of game
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: ResMut<Windows>) {
    // Watches for changes in files
    asset_server.watch_for_changes().unwrap();

    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // position window to top left
    let mut window = windows.get_primary_mut().unwrap();

    // Creates a resource that can later be used
    commands.insert_resource(WinSize {
        w: window.width(),
        h: window.height(),
    });

    //TODO: init snake head and board as resources
    // default snake for testing
    let points = vec![Point{x:7,y:7}, Point{x:7,y:8}, Point{x:7,y:9}, Point{x:6,y:9}];
    commands.insert_resource(Snake {points: points});
    commands.insert_resource(Direction{ d: Movement::Right });
}

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .insert_resource(WindowDescriptor {
        title: "Rusty Snake".to_string(),
        width: 676.0,
        height: 676.0,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .add_system_set(
        SystemSet::new()
            .with_run_criteria(FixedTimestep::step(1.0)) // controls speed of snake
            .with_system(snake_movement)
            .with_system(draw_system),
    )
    .add_system(keyboard_control)
    .add_system(food_system)
    .add_system(score_system)
    .add_system(open_menu_system)
    .run();
}
