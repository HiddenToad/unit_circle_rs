use nannou::prelude::*;
use nannou::winit::event::VirtualKeyCode;

fn main() {
    //run our app
    nannou::app(model).run();
}

//the radius of the unit circle
const RADIUS: f32 = 200.;


//the unit circle itself
struct UnitCircle {
    //the angle of theta
    angle: f32,
}


//our app's data model,
//consists of the window, the unit circle, 
//and stores the position of the points that
//make it up for reuse
struct Model {
    _window: window::Id,
    circle: UnitCircle,
    points: [(Vec2, Rgb<u8>); 361],
}

//construct the model
fn model(app: &App) -> Model {
    Model {
        _window: app.new_window().event(event).view(view).build().unwrap(),
        circle: UnitCircle { angle: 0. },
        points: (*(0..=360)
            .map(|i| (to_circle_point(i as f32), BLACK))
            .collect::<Vec<_>>()
            .as_slice())
        .try_into()
        .unwrap(),
    }
}

//on receiving an input event, run
fn event(_: &App, model: &mut Model, event: WindowEvent) {
    match event {
        WindowEvent::KeyPressed(key) => {
            match key {
                //if keycode is A move left
                VirtualKeyCode::A => match model.circle.angle as u32 {
                    0..=90 | 271..=360 => {
                        if model.circle.angle >= 1. {
                            model.circle.angle -= 1.;
                        } else {
                            model.circle.angle = 359.;
                        }
                    }
                    _ if model.circle.angle <= 360. => {
                        model.circle.angle += 1.;
                    }
                    _ => {
                        unreachable!()
                    }
                },
                //if keycode is d move right
                VirtualKeyCode::D => match model.circle.angle as u32 {
                    0..=90 | 271..=360 => {
                        model.circle.angle += 1.;
                    }
                    _ if model.circle.angle <= 360. => {
                        if model.circle.angle >= 1. {
                            model.circle.angle -= 1.;
                        } else {
                            //wrap to positive, 360
                            model.circle.angle = 359.;
                        }
                    }
                    _ => {
                        unreachable!()
                    }
                },
                _ => {}
            }
            //force angle to wrap to 360
            model.circle.angle %= 360.;
        }
        _ => {}
    }
}

//given circle point #n, return its x and y
fn to_circle_point(i: f32) -> Vec2 {
    let rad = deg_to_rad(i);
    Vec2::new(rad.sin() * RADIUS, rad.cos() * RADIUS)
}

//creates the app's view once per frame.
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    //the point at which the theta line ends
    let point = to_circle_point(model.circle.angle);

    //x axis line
    draw.line()
        .weight(3.)
        .start(pt2(-RADIUS, 0.))
        .end(pt2(RADIUS, 0.));

    //y axis line
    draw.line()
        .weight(3.)
        .start(pt2(0., -RADIUS))
        .end(pt2(0., RADIUS));

    //circle
    draw.polyline().weight(3.).points_colored(model.points);

    //rotating line
    draw.line()
        .weight(3.)
        .start(pt2(0., 0.))
        .end(point)
        .color(RED);

    //sin line
    draw.line()
        .weight(3.)
        .start(point)
        .end(pt2(point.x, 0.))
        .color(BLUE);

    //cos line
    draw.line()
        .weight(3.)
        .start(pt2(0., 0.))
        .end(pt2(point.x, 0.))
        .color(ORANGE);

    //angle text
    draw.text(&format!("angle: {}°", model.circle.angle))
        .color(RED)
        .font_size(25)
        .xy(app.window_rect().top_left() + pt2(160., -20.))
        .left_justify()
        .width(300.);

    //sin text
    draw.text(&format!("sin: {}", deg_to_rad(model.circle.angle).sin()))
        .color(BLUE)
        .font_size(25)
        .xy(app.window_rect().top_left() + pt2(160., -50.))
        .left_justify()
        .width(300.);

    //cos text
    draw.text(&format!("cos: {}", deg_to_rad(model.circle.angle).cos()))
        .color(ORANGE)
        .font_size(25)
        .xy(app.window_rect().top_left() + pt2(160., -80.))
        .left_justify()
        .width(300.);

    //write all the contents of the draw handle to our app's window
    draw.to_frame(app, &frame).unwrap();
}
