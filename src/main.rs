use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

fn main () {
    let event_loop = EventLoop::new();

    let builder = WindowBuilder::new();

    builder.build(&event_loop).expect("Window did not build");

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
    })
}
