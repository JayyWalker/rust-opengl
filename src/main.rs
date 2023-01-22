use WindowEvent::CloseRequested;
use winit::dpi::LogicalSize;
use winit::event::{DeviceId, Event, WindowEvent};
use winit::event::WindowEvent::CursorMoved;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

fn main () {
    let event_loop = EventLoop::new();

    let builder = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(512, 512))
        .with_title("OpenGL sandbox");

    // builder.build(&event_loop).expect("Window did not build");
    let window = builder.build(&event_loop).unwrap();

    let size = window.primary_monitor().unwrap().size();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::WindowEvent {
                event: CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                control_flow.set_exit();
            },
            Event::WindowEvent {
                event: CursorMoved { position, .. },
                ..
            } => {
                println!("Cursor position x: {}  y: {}", position.x, position.y);
            },
            Event::RedrawRequested(_) => {
                println!("Redraw Requested!");
                println!("width: {} height: {}", size.width, size.height);
            },
            _ => ()
        }
    })
}
