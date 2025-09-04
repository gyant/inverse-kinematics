#[derive(Debug)]
struct RobotArm {
    origin: Point,
    joint: Joint,
}

impl RobotArm {
    //      |
    //      |
    //      |
    //      |
    //     ( )
    fn new() -> RobotArm {
        return RobotArm {
            origin: Point { x: 0, y: 0 },
            joint: Joint {
                min_angle: 0,
                max_angle: 180,
                current_angle: 90,
                segment: Some(Box::new(Segment { length: 10 })),
            },
        };
    }
}

#[derive(Debug)]
struct Segment {
    length: i64,
}

#[derive(Debug)]
struct Joint {
    min_angle: u8,
    max_angle: u8,
    current_angle: u8,
    segment: Option<Box<Segment>>,
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn main() {
    let robot_arm = RobotArm::new();
    println!("{:?}", robot_arm);

    // Point straight ahead at 90 degress at len 10 should be (0, 10)
    // y: sin(90) == 1
    // x: cos(90) == 0
    //
    // len * sin(90) == 10
    // len * cos(90) == 0
}
