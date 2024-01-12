slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_calculate(move |ethanol_value, gas_value| {
        let ui = ui_handle.unwrap();
        let ethanol: f64 = ethanol_value
            .replace(",", ".")
            .trim()
            .parse()
            .unwrap_or(0.0);
        let gas: f64 = gas_value.replace(",", ".").trim().parse().unwrap_or(0.0);
        print!("{} {} {:.2}", ethanol, gas, ethanol / gas);
        let result = if ethanol / gas < 0.7 {
            "You should use Ethanol"
        } else if ethanol / gas > 0.7 {
            "You should use Gasoline"
        } else {
            "In this case, you can use any of them"
        };

        ui.set_what_should_i_use(result.into());
    });

    ui.run()
}
