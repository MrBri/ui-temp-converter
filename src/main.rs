use slint::ComponentHandle;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_convert_c_f({
        let ui_handle = ui.as_weak(); // Capture a weak reference

        move |input| {
            if let Some(ui) = ui_handle.upgrade() { // Upgrade if reference is alive
                let cel: f32 = input.trim().parse().unwrap_or(0.0);
                let fah = (cel * 1.8) + 32.0;
                ui.set_c_f(fah.round());
                println!("get c_f: {}, input: {}", ui.get_c_f(), input);
            } 
        }
    });
    ui.on_convert_f_c({
        let ui_handle = ui.as_weak(); // Capture a weak reference

        move |input| {
            if let Some(ui) = ui_handle.upgrade() { // Upgrade if reference is alive
                let fah: f32 = input.trim().parse().unwrap_or(0.0);
                let cel = (fah - 32.0) / 1.8; 
                ui.set_f_c(cel.round());
                println!("get f_c: {}, input: {}", ui.get_f_c(), input);
            } 
        }
    });

    ui.run()
}
