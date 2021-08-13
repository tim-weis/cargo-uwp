fn main() {
    windows::build! {
        Windows::Win32::System::Com::CoInitializeEx,
        Windows::UI::Xaml::Controls::Button,
        Windows::UI::Xaml::{Application, Window},
    };
}
