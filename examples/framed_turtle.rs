extern crate drawille;

use drawille::Turtle;

fn main() {
    let mut turtle = Turtle::new(50., 0.);
    //turtle.up();
    turtle.down();
    for n in 0..160 {
        turtle.forward(16. - (n as f32)/10.);
        turtle.right(16.);
    }
    let leftmost_x = 12;
    let topmost_y = 12;
    let frame_width = 30;
    let frame_height = 15;
    println!("{}", turtle.selected_frame(leftmost_x, topmost_y, frame_width, frame_height));
}