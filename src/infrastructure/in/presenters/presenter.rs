use application::interfaces::r#in::presenter_core::Presenter as PresenterCore;

pub trait Presenter<T, U>: PresenterCore<T> {
    fn get_output(&self) -> Result<U, String>;
}
