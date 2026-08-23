slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let w = MainWindow::new()?;
    let weak = w.as_weak();
    w.on_picked(move |id| {
        println!("picked: {id}");
        if let Some(w) = weak.upgrade() {
            w.set_status(format!("picked: {id}").into());
        }
    });
    w.run()
}
