#[cfg(not(target_arch = "wasm32"))]
mod egui_dialog {
    use egui_file_dialog::FileDialog;
    use std::path::PathBuf;
    pub enum HtmlInputElement {}
    pub struct CompatFileDialog {
        native_file_dialog: Option<FileDialog>,
    }
    impl CompatFileDialog {
        pub fn new(_html_file_input: Option<HtmlInputElement>) -> Self {
            Self {
                native_file_dialog: Some(FileDialog::new()),
            }
        }

        pub fn placeholder() -> Self {
            Self {
                native_file_dialog: None,
            }
        }

        pub fn pick_file(&mut self) {
            self.native_file_dialog.as_mut().expect("native_file_dialog not instantiated").pick_file();
        }

        pub fn take_picked(&mut self) -> Option<PathBuf> {
            self.native_file_dialog.as_mut().expect("native_file_dialog not instantiated").take_picked()
        }

        pub fn update(&mut self, ui: &mut egui::Ui) -> () {
            #[cfg(not(target_arch = "wasm32"))]
            self.native_file_dialog.as_mut().expect("native_file_dialog not instantiated").update(&ui);
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod web {
    use std::path::PathBuf;
    pub type HtmlInputElement = web_sys::HtmlInputElement;
    pub struct CompatFileDialog {
        html_file_input: Option<HtmlInputElement>,
    }
    impl CompatFileDialog {
        pub fn new(html_file_input: Option<HtmlInputElement>) -> Self {
            Self { html_file_input }
        }

        pub fn placeholder() -> Self {
            Self { html_file_input: None }
        }

        pub fn pick_file(&mut self) {
            self.html_file_input.as_ref().expect("html_file_input not instantiated").click();
        }

        pub fn take_picked(&mut self) -> Option<PathBuf> {
            let file_input = self.html_file_input.as_mut().expect("html_file_input not instantiated");
            let file_name = file_input.files()?.get(0).map(|file: web_sys::File| file.name());
            file_input.set_value(""); // TODO: this should return Some only once, and then switch back to None

            // TODO: currently this only returns a filename, it would be useful to have some sort of file "handle", bc you cant read from just a filename in web
            file_name.map(PathBuf::from)
        }

        pub fn update(&mut self, _ui: &mut egui::Ui) -> () {}
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub type CompatFileDialog = egui_dialog::CompatFileDialog;
#[cfg(not(target_arch = "wasm32"))]
pub type HtmlInputElement = egui_dialog::HtmlInputElement;

#[cfg(target_arch = "wasm32")]
pub type HtmlInputElement = web::HtmlInputElement;
#[cfg(target_arch = "wasm32")]
pub type CompatFileDialog = web::CompatFileDialog;
