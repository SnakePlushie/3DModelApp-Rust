use infrastructure::r#in::controllers::{
    app::App,
    mapper::{key_mapper::KeyMapper, window_event_mapper::WindowEventMapper},
};
use winit::{error::EventLoopError, event_loop::EventLoop};

fn main() {
    println!("Hello World");
    let _ = start().inspect_err(|err| println!("App Error: {err:#?}"));
}

fn start() -> Result<(), EventLoopError> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let event_loop = EventLoop::with_user_event().build()?;

    let key_mapper = KeyMapper;
    let window_event_mapper = WindowEventMapper::new(key_mapper);
    let mut app = App::new(window_event_mapper);

    event_loop.run_app(&mut app)?;

    Ok(())
}
