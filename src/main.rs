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

enum Movement {
    // Variants
    Up, Down, Left, Right
}

fn snake_movement(
    mut commands: Commands,
    win_size: Res<WinSize>,
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

    println!("{:?}", snake.points);
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
            .with_system(snake_movement),
    )
    .run();
}
