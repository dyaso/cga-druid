#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use druid::kurbo::{BezPath, Circle};
use druid::piet::{FontFamily, FontStyle, FontWeight, ImageFormat, InterpolationMode};

use druid::kurbo;

use druid::widget::prelude::*;
use druid::{
    Affine, AppLauncher, ArcStr, Color, Data, FontDescriptor, LocalizedString, MouseEvent, Selector, Target,
    Point as DruidPoint, Rect, TextLayout, WindowDesc,
};


const FILES_CHANGED: Selector<notify::Event> = Selector::new("drawing-app.files-changed");

//use klein::{ApplyOp, Line, Plane, Point, Rotor, Translator};

mod cga2d;
use cga2d::*;

#[derive(Default, Debug)]
struct CustomWidget {
    lower_y: f64,
    upper_y: f64,
    left_x: f64,
    right_x: f64,

    // lower_plane: Plane,
    // upper_plane: Plane,
    // left_plane: Plane,
    // right_plane: Plane,

    //center_point: Rc<PGA3D>,
    // center_point: Point,
    left: f64,
    top: f64,
    scale: f64,

    window_pixels: kurbo::Size,
}

// #[derive(Copy, Clone)]
// struct FlatlandPoint {
//     line: Line,
// }

// impl From<Point> for FlatlandPoint {
//     fn from(p: Point) -> FlatlandPoint {
//         FlatlandPoint { line: p & Point::new(0., 0., 1.) }
//     }
// }

// // impl From<&FlatlandPoint> for Line {
// //     fn from(zp_pt: &FlatlandPoint) -> Line {
// //         zp_pt.line
// //     }
// // }

// impl From<&FlatlandPoint> for Point {
//     fn from(zp_pt: &FlatlandPoint) -> Point {
//         zp_pt.line ^ Plane::new(0.,0.,1.,0.)
//     }
// }
// impl From<FlatlandPoint> for Point {
//     fn from(zp_pt: FlatlandPoint) -> Point {
//         zp_pt.line ^ Plane::new(0.,0.,1.,0.)
//     }
// }

// // use std::ops::{BitXor, BitAnd};

// // impl BitXor<FlatlandPoint> for FlatlandPoint {
// //     type Output = Line;
// //     #[inline]
// //     fn bitxor(self, rhs: FlatlandPoint) -> Self::Output {
// //         Point::from(self) & Point::from(rhs)
// //     }
// // }

// // impl BitAnd<FlatlandPoint> for FlatlandPoint {
// //     type Output = Line;
// //     #[inline]
// //     fn bitand(self, rhs: FlatlandPoint) -> Self::Output {
// //         Point::from(self) & Point::from(rhs)
// //     }
// // }

// impl FlatlandPoint {

//     pub fn new<X: Into<f32>, Y: Into<f32>>(x: X, y: Y) -> FlatlandPoint {
//         FlatlandPoint::from(Point::new(x.into(), y.into(), 0.))
//     }
// }

// struct FlatlandLine {
//     plane: Plane,
// }

// impl From<Line> for FlatlandLine {
//     fn from(l: Line) -> FlatlandLine {
//         FlatlandLine { plane: l & Point::new(0., 0., 1.) }
//     }
// }


impl CustomWidget {
    fn to_druid_point(&self, p: &Conformal2D) -> DruidPoint {
        let pn = p; //.normalized();

        let x = 0.;
        let y = 0.;

        DruidPoint::new(
            (x as f64 - self.left) / self.scale,
            (self.top - y as f64) / self.scale,
        )
    }

    fn new() -> CustomWidget {
        CustomWidget {
            lower_y: -2.,
            upper_y: 2.,
            left_x: -2.,
            right_x: 2.,
            ..Default::default()
        }
    }
    pub fn set_window_boundary_planes(&mut self, state: &State, window: &kurbo::Size) {
        let desired_width = self.right_x - self.left_x;
        let desired_height = self.upper_y - self.lower_y;
        let desired_aspect_ratio = desired_width / desired_height;

        let center_x = (self.right_x + self.left_x) / 2.;
        let center_y = (self.upper_y + self.lower_y) / 2.;

        self.window_pixels = *window;
        let window_aspect_ratio = window.width / window.height;

        self.top = self.upper_y;
        self.left = self.left_x;
        let mut right = self.right_x;
        let mut bottom = self.lower_y;

        if window_aspect_ratio > desired_aspect_ratio {
            // actual window is wider than desired viewport
            self.scale = desired_height / window.height;
            let half_width = self.scale * 0.5 * window.width;
            right = center_x + half_width;
            self.left = center_x - half_width;
        } else {
            // actual window is taller than desired viewport
            self.scale = desired_width / window.width;
            let half_height = self.scale * 0.5 * window.height;
            self.top = center_y + half_height;
            bottom = center_y - half_height;
        }

        // self.lower_plane = Plane::new(0., 1., 0., bottom as f32);
        // self.upper_plane = Plane::new(0., 1., 0., self.top as f32);
        // self.left_plane = Plane::new(1., 0., 0., self.left as f32);
        // self.right_plane = Plane::new(1., 0., 0., right as f32);

        // self.center_point = Point::new(center_x as f32, center_y as f32, 0.);
    }

    // fn dist(a: &Point, b: &Point) -> f32 {
    //     let d = (a.normalized() & b.normalized()).norm();

    //     return if f32::is_nan(d) { 1000000. } else { d };
    // }

    // pub fn draw_line(&self, ctx: &mut PaintCtx, line: &Line) {
        // let mut intersections = Vec::<Point>::new();
        // intersections.push(*line ^ self.lower_plane);
        // intersections.push(*line ^ self.upper_plane);
        // intersections.push(*line ^ self.left_plane);
        // intersections.push(*line ^ self.right_plane);

        // intersections.sort_by(|a, b| {
        //     CustomWidget::dist(a, &self.center_point)
        //         .partial_cmp(&CustomWidget::dist(b, &self.center_point))
        //         .unwrap()
        // });

        // let end1 = &intersections[0].normalized();
        // let end2 = &intersections[1].normalized();
        // // self.draw_point(ctx, end1);
        // // self.draw_point(ctx, end2);
        // let mut path = BezPath::new();
        // path.move_to(DruidPoint::new(
        //     ((end1.x() as f64 - self.left) / self.scale),
        //     ((self.top - end1.y() as f64) / self.scale),
        // ));
        // path.line_to(DruidPoint::new(
        //     ((end2.x() as f64 - self.left) / self.scale),
        //     ((self.top - end2.y() as f64) / self.scale),
        // ));
        // let stroke_color = Color::rgb8(150, 150, 150);
        // ctx.stroke(path, &stroke_color, 1.0);
    // }

    // pub fn draw_point(&self, ctx: &mut PaintCtx, highlight: bool) {
    //     let point = Point::from(zeepoint);
    //     let mut fill_color = Color::rgba8(0xa3, 0xff, 0xff, 0xFF);
    //     if highlight {
    //         fill_color = Color::rgba8(0xff, 0x70, 0x70, 0xff);
    //     }
    //     let dp = DruidPoint::new(0.,0.,
    //                 // ((point.x() as f64 - self.left) / self.scale),
    //                 // ((self.top - point.y() as f64) / self.scale),
    //             );

    //     ctx.fill(
    //         Circle::new(
    //             dp,
    //             15.0,
    //         ),
    //         &fill_color,
    //     );
    //     ctx.fill(
    //         Circle::new(
    //             dp,
    //             5.0,
    //         ),
    //         &Color::WHITE,
    //     );
    // }

    pub fn mouse_move(&self, mouse: &MouseEvent, data: &mut State) {
        // let x_portion = (mouse.pos.x / self.window_pixels.width);
        // let x_coord = self.left_plane.x() * self.left_plane.d() * (1. - x_portion as f32)
        //     + (self.right_plane.x() * self.right_plane.d()) * x_portion as f32;
        // let y_portion = (mouse.pos.y / self.window_pixels.height);
        // let y_coord = self.upper_plane.y() * self.upper_plane.d() * (1. - y_portion as f32)
        //     + self.lower_plane.y() * self.lower_plane.d() * y_portion as f32;
        // // let mouse_point = (x_plane^y_plane.normalized()^PGA3D::e3());
        // // let mouse_y = mouse.pos.y;
        // // let y_plane =

        // let mouse = Point::new(x_coord, y_coord, 0.);
        // data.mouse_over = None;
        // for (i, p) in data.points.iter().enumerate() {
        //     // println!("mouse point {} {}",i, (mouse & *p).norm());
        //     if (mouse & Point::from(p)).norm() < 0.07 {
        //         println!("over point {}", i);
        //         data.mouse_over = Some(i);
        //     }
        // }

        // println!("mosue pos {} {}",mouse_point.get032(), mouse_point.get013())
    }
}

use std::rc::Rc;
use std::sync::Arc;

#[derive(Clone, Data, Default)]
struct State {
    points: Arc<Vec<Conformal2D>>,
    mouse_over: Option<usize>,
    // mesh: Rc<Vec<Point>>,
    indices: Arc<Vec<Vec<usize>>>,
    time: f64,
    repaint: Arc<bool>,
}

impl Widget<State> for CustomWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut State, _env: &Env) {
        match event {
            Event::WindowConnected => {
                ctx.request_focus();
                //ctx.request_anim_frame();
            }
            Event::KeyDown(e) => {
                println!("key down event {:?}", e);
            }
            Event::MouseMove(e) => {
                let old = data.mouse_over;
                self.mouse_move(e, data);
                if data.mouse_over != old {
                    ctx.request_paint();
                }
            }
            Event::AnimFrame(_interval) => {
                data.time += 0.01;
                ctx.request_paint();
                // ctx.request_anim_frame();
            }

            Event::Command(cmd) if cmd.is(FILES_CHANGED) => {
                println!("{}Files changed{}: {:?}", CYAN_TEXT, RESET_TEXT, cmd.get_unchecked(FILES_CHANGED))
                // data.0 = cmd.get_unchecked(SET_COLOR).clone();
                // ctx.request_paint();
            }

            _ => {
                println!("Unhandled {}widget event{}: {:?}", MAGENTA_TEXT,RESET_TEXT,event);
            }
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &State, _env: &Env) {
        match event {
            LifeCycle::Size(s) => {
                self.set_window_boundary_planes(data, s);
            }
            LifeCycle::WidgetAdded => {
                ctx.register_for_focus();
            }
            _ => println!("Unhandled lifecycle event: {:?}", event),
        }
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &State, _data: &State, _env: &Env) {

        // println!("update event: {}", );
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &State,
        _env: &Env,
    ) -> Size {
        // BoxConstraints are passed by the parent widget.
        // This method can return any Size within those constraints:
        // bc.constrain(my_size)
        //
        // To check if a dimension is infinite or not (e.g. scrolling):
        // bc.is_width_bounded() / bc.is_height_bounded()
        bc.max()
    }

    // The paint method gets called last, after an event flow.
    // It goes event -> update -> layout -> paint, and each method can influence the next.
    // Basically, anything that changes the appearance of a widget causes a paint.
    fn paint(&mut self, ctx: &mut PaintCtx, data: &State, env: &Env) {
        // Let's draw a picture with Piet!
println!("PAINTING {:#?}", self);
        // Clear the whole widget with the color of your choice
        // (ctx.size() returns the size of the layout rect we're painting in)
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &Color::BLACK);

        // // for p in (&data.points).borrow().iter() {
        // for p in (&data.points).iter() {
        //     println!("label: {}", p.label);
        // }

        // Note: ctx also has a `clear` method, but that clears the whole context,
        // and we only want to clear this widget's area.

        // Create an arbitrary bezier path
        // let mut path = BezPath::new();
        // path.move_to(Point::ORIGIN);
        // path.quad_to((80.0, 90.0), (size.width, size.height));
        // // Create a color
        // let stroke_color = Color::rgb8(255,0, 255);
        // // Stroke the path with thickness 1.0
        // ctx.stroke(path, &stroke_color, 1.0);

        for (i, p) in data.points.iter().enumerate() {
            let mut mouse_is_over = false;
            if let Some(idx) = data.mouse_over {
                if idx == i {
                    mouse_is_over = true;
                }
            }

            // self.draw_point(ctx, p, mouse_is_over);

        }

        // let l = p1 & p2;

        // self.draw_point(ctx, &p1, over1);
        // self.draw_point(ctx, p2, over2);

            // let l2 = &(Point::new(0.3, 0., 0.) & Point::new(0., 1., 0.));

            // let Flatland = Plane::new(0., 0., 1., 0.);

            // let z1 = Point::new(0., 0., 1.);
//        let pz = p1 + z1;
        // println!("{}", p1);
        // println!("{}", pz);

//        let trans = Translator::translator(1., 0., 0., 1.);

//        let plan = l | Flatland;
        // let plan = (p1 & p2 & Point::direction(0., 0., 1.));

        // self.draw_line(ctx, &l);
        // self.draw_line(ctx, l2);

//        let int = *l2 ^ plan;
        // self.draw_point(ctx, &int.normalized(), false);

        // let zAxis =
        //let p = *l & (*l2 ^ Point::direction(0., 0., 1.));

        // println!("{}", plan);
        // self.draw_point(ctx, &p);

        //         // 0 = same plane (intersect at origin)
        //         // -1e02 parallel planes separated but parallel
        //         // 0.0995037e01 + -0.9950371e02 + -0.0995037e12 intersect in line

        //         // Rectangles: the path for practical people
        // //        let rect = Rect::from_origin_size((10., 10.), (100., 100.));
        //         // Note the Color:rgba8 which includes an alpha channel (7F in this case)
        let fill_color = Color::rgba8(0xa3, 0xa3, 0xa3, 0xFF);
        // //        ctx.fill(rect, &fill_color);

        //         ctx.stroke(
        //             Circle::new(Point::new(data.x as f64, data.y as f64), 15.0),
        //             &fill_color,
        //             5.0,
        //         );
        // let r = Rotor::rotor(data.time as f32, 1., -1., 0.5);

        for p in data.indices.iter() {
            // println!("drawing {} {} {}", p[0], p[1], p[2]);
            let mut path = BezPath::new();
            // path.move_to(self.to_druid_point(&r.apply_to(data.mesh[p[0]])));
            // path.line_to(self.to_druid_point(&r.apply_to(data.mesh[p[1]])));
            // path.line_to(self.to_druid_point(&r.apply_to(data.mesh[p[2]])));
            // path.line_to(self.to_druid_point(&r.apply_to(data.mesh[p[0]])));
            let stroke_color = Color::rgb8(240, 240, 240);
            ctx.stroke(path, &stroke_color, 4.0);
        }

        // Text is easy; in real use TextLayout should be stored in the widget
        // and reused.
        let mut layout = TextLayout::<ArcStr>::from_text("conformal geometry 2d"); //data.to_owned());
        layout.set_font(
            FontDescriptor::new(FontFamily::SANS_SERIF)
                .with_size(24.0) //.with_weight(FontWeight::BOLD)
                .with_style(FontStyle::Italic),
        );
        layout.set_text_color(fill_color);
        //        layout.set_text_style(FontStyle::Italic);
        layout.rebuild_if_needed(ctx.text(), env);

        // Let's rotate our text slightly. First we save our current (default) context:
        ctx.with_save(|ctx| {
            // Now we can rotate the context (or set a clip path, for instance):
            ctx.transform(Affine::rotate(0.0));
            layout.draw(ctx, (80.0, 40.0));
        });
        // When we exit with_save, the original context's rotation is restored

        //drawtext(ctx);
        // let layout2 = ctx
        // .text()

        // let mut moo: () = layout2;
        // .new_text_layout("Helloo piet!");
        // .font(FontFamily::SYSTEM_UI, 24.0)
        // .default_attribute(FontStyle::Italic)
        // .default_attribute(FontWeight::BOLD)
        // .default_attribute(TextAttribute::TextColor(RED_ALPHA))
        // .build()?;

        //     let w: f64 = layout2.size().width;
        //     rc.draw_text(&layout2, (80.0, 10.0));

        //     rc.stroke(Line::new((80.0, 12.0), (80.0 + w, 12.0)), &RED_ALPHA, 1.0);

        //     rc.with_save(|rc| {
        //         rc.transform(Affine::rotate(0.1));
        //         rc.draw_text(&layout2, (80.0, 10.0));
        //         Ok(())
        //     })?;

        // Let's burn some CPU to make a (partially transparent) image buffer
        let image_data = make_image_data(256, 256);
        let image = ctx
            .make_image(256, 256, &image_data, ImageFormat::RgbaSeparate)
            .unwrap();
        // The image is automatically scaled to fit the rect you pass to draw_image
        ctx.draw_image(&image, size.to_rect(), InterpolationMode::Bilinear);
    }
}

// fn draw_triangle(ctx: &mut PaintCtx, idx: usize, vertices: Vec<Point>, indices: Vec<Vec<usize>>) {}
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Result};

mod scripting;
//use crate::scripting;

const RESET_TEXT: &str = "\u{001b}[0m";

const BLACK_TEXT: &str = "\u{001b}[30m";
const RED_TEXT: &str = "\u{001b}[31m";
const GREEN_TEXT: &str = "\u{001b}[32m";
const YELLOW_TEXT: &str = "\u{001b}[33m";
const BLUE_TEXT: &str = "\u{001b}[34m";
const MAGENTA_TEXT: &str = "\u{001b}[35m;";
const CYAN_TEXT: &str = "\u{001b}[36m";
const WHITE_TEXT: &str = "\u{001b}[37m";
const BRIGHT_BLACK_TEXT: &str = "\u{001b}[30;1m";
const BRIGHT_RED_TEXT: &str = "\u{001b}[31;1m";
const BRIGHT_GREEN_TEXT: &str = "\u{001b}[32;1m";
const BRIGHT_YELLOW_TEXT: &str = "\u{001b}[33;1m";
const BRIGHT_BLUE_TEXT: &str = "\u{001b}[34;1m";
const BRIGHT_MAGENTA_TEXT: &str = "\u{001b}[35;1m";
const BRIGHT_CYAN_TEXT: &str = "\u{001b}[36;1m";
const BRIGHT_WHITE_TEXT: &str = "\u{001b}[37;1m";
const BLACK_BG: &str = "\u{001b}[40m";
const RED_BG: &str = "\u{001b}[41m";
const GREEN_BG: &str = "\u{001b}[42m";
const YELLOW_BG: &str = "\u{001b}[43m";
const BLUE_BG: &str = "\u{001b}[44m";
const MAGENTA_BG: &str = "\u{001b}[45m";
const CYAN_BG: &str = "\u{001b}[46m";
const WHITE_BG: &str = "\u{001b}[47m";

const BOLD: &str = "\u{001b}[1m";
const UNDERLINED: &str = "\u{001b}[4m";
const INVERTED: &str = "\u{001b}[7m";

    // Reset: \u001b[0m


use std::thread;

// impl State {
//     pub fn repaint(&mut self) {
//         self.repaint = true;
//     }
// }

pub fn main() -> Result<()> {
    let window = WindowDesc::new(|| CustomWidget::new()).title(
        LocalizedString::new("custom-widget-demo-window-title").with_placeholder("CGA rust"),
    );


    let (tx, rx) = std::sync::mpsc::channel();

//    let () = rx; //std::sync::mpsc::Receiver

    let mut rep = Arc::<bool>::new(true);

    let mut s = State {
        repaint: Arc::clone(&rep),
        ..Default::default()
    };

    let script = crate::scripting::rhai::RunningScript::new("../diagram drawing scripts/test.rhai".into());

//https://github.com/notify-rs/notify/blob/main/examples/monitor_raw.rs

    // // Automatically select the best implementation for your platform.
    // let mut watcher: RecommendedWatcher = Watcher::new_immediate(|res| {
    //     match res {
    //         Ok(event) => {
    //             println!("{}watch{} event: {:?}", RED_TEXT, RESET_TEXT, event);
    //             s.repaint = true;
    //         },
    //         Err(e) => println!("watch error: {:?}", e),
    //     }
    // })?;
    let mut watcher: RecommendedWatcher = Watcher::new_immediate(move |res| tx.send(res).unwrap())?;
    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch("../diagram drawing scripts/", RecursiveMode::Recursive)?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    //watcher.watch(path, RecursiveMode::Recursive)?;

//    let mut rep = &Arc::get_mut(&mut s.repaint).unwrap();

    let launcher = AppLauncher::with_window(window);
    let event_sink = launcher.get_external_handle();


    thread::spawn(move || {
        for res in rx {
            match res {
                Ok(event) => {
                    println!("changed: {:?}", event);
                    // let () = event;
                    if event_sink.submit_command(FILES_CHANGED, event, Target::Auto).is_err() {
                        println!("Submitting files-changed event to Druid window failed");
                    }
                },
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    });
println!("current dir: {:?}", std::env::current_dir().unwrap());


//    let mut ps = Rc::get_mut(&mut s.points).unwrap();
    // let mut ps = s.points;

    let mut p = Conformal2D::up(1.,1.,);
    println!("POINTS {}", p);

    p = p * 2.;
    println!("POINTS2 {}", p);

    // println!("e3 ^ 2 = {} {}", Conformal2D::e3(), Conformal2D::e3() * Conformal2D::e3());
    // println!("e4 ^ 2 = {} {}", Conformal2D::e4(), Conformal2D::e4() * Conformal2D::e4());

   // let c = (Conformal2D::no() | &p) * (&p).Conjugate();// 4
   // let c = &p * &(p.Conjugate());
   let c = &p * &(p.Dual());
    // let c = (Conformal2D::no() | &p) * (&p).Reverse();// -4
  //  let c = (Conformal2D::no() | &p) * (&p).Reverse();// -4

// println!("CCC {}", c);
// println!("ppp {}", (Conformal2D::ni() | p).get_s());
    println!("POINTS2 {}, {}", p.x(), p.y());


    //s.points.push(p);
    
    // ps.push(FlatlandPoint::new(0., 0.6));
    // ps.push(FlatlandPoint::new(0.94, 0.));

    // let y1 = Point::new(0., 1., 0.);
    // let p1 = Rotor::rotor(f32::acos(-1. / 3.), 1., 0., 0.).apply_to(y1);
    // let r = Rotor::rotor(f32::acos(-0.5), 0., 1., 0.);
    // let p2 = r.apply_to(p1);

    // let mut mesh = Rc::get_mut(&mut s.mesh).unwrap();
    // mesh.push(y1);
    // mesh.push(p1);
    // mesh.push(p2);
    // mesh.push(r.apply_to(p2));

    // let mut indices = Rc::get_mut(&mut s.indices).unwrap();
    // indices.push(vec![0, 1, 2]);
    // indices.push(vec![0, 2, 3]);
    // indices.push(vec![0, 3, 1]);
    // indices.push(vec![1, 3, 2]);


    launcher
        .use_simple_logger()
        .launch(s)
        // .launch("Druid + Piet".to_string())
        .expect("Conformal geometric algebra drawing app launch failed");

    Ok(())
}

fn make_image_data(width: usize, height: usize) -> Vec<u8> {
    let mut result = vec![0; width * height * 4];
    for y in 0..height {
        for x in 0..width {
            let ix = (y * width + x) * 4;
            result[ix] = x as u8;
            result[ix + 1] = y as u8;
            result[ix + 2] = !(x as u8);
            result[ix + 3] = 127;
        }
    }
    result
}
