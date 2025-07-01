mod app;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let opt = eframe::NativeOptions::default();
    eframe::run_native("app", opt, Box::new(|cc| Ok(Box::new(app::App::new(cc)))))
}

#[cfg(target_arch = "wasm32")]
fn main() {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    wasm_bindgen_futures::spawn_local(async {
        use eframe::wasm_bindgen::JsCast as _;

        let document = web_sys::window().unwrap().document().unwrap();

        let canvas = document
            .get_element_by_id("app")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let opt = eframe::WebOptions::default();
        let result = eframe::WebRunner::new()
            .start(canvas, opt, Box::new(|cc| Ok(Box::new(app::App::new(cc)))))
            .await;

        match result {
            Ok(()) => document.get_element_by_id("loading-text").unwrap().remove(),
            Err(e) => {
                document
                    .get_element_by_id("loading-text")
                    .unwrap()
                    .set_inner_html("<p>App crashed!!!</p>");
                panic!("Failed to start eframe: {e:?}");
            }
        }
    });
}
