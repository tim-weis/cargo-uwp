#![windows_subsystem = "windows"]

use std::convert::TryFrom;

use bindings::*;
use windows::*;

use bindings::{
    Windows::ApplicationModel::Activation::LaunchActivatedEventArgs,
    Windows::Win32::System::Com::*,
    Windows::UI::Xaml::Controls::Button,
    Windows::UI::Xaml::{
        Application, ApplicationInitializationCallback, HorizontalAlignment, RoutedEventHandler,
        VerticalAlignment, Window,
    },
};

#[implement(
    extend Windows::UI::Xaml::Application,
    override OnLaunched
)]
struct MyApp();

#[allow(non_snake_case)]
impl MyApp {
    fn OnLaunched(&self, _: &Option<LaunchActivatedEventArgs>) -> Result<()> {
        let button = Button::new()?;
        button.SetContent(IInspectable::try_from("Click Me")?)?;
        button.SetHorizontalAlignment(HorizontalAlignment::Center)?;
        button.SetVerticalAlignment(VerticalAlignment::Center)?;
        button.Click(RoutedEventHandler::new(|sender, _args| {
            if let Some(button) = sender {
                button
                    .cast::<Button>()?
                    .SetContent(IInspectable::try_from("Clicked! 🦀")?)?;
            }
            Ok(())
        }))?;

        let window = Window::Current()?;
        window.SetContent(&button)?;
        window.Activate()
    }
}

fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(std::ptr::null_mut(), COINIT_MULTITHREADED)?;
    }
    Application::Start(ApplicationInitializationCallback::new(|_| {
        MyApp().new()?;
        Ok(())
    }))
}
