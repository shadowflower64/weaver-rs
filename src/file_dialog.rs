#[cfg(not(target_arch = "wasm32"))]
mod egui_dialog {
    use egui_file_dialog::FileDialog;
    use std::{fs::File, io::Read, path::PathBuf};
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

        pub fn take_picked(&mut self) -> Option<ReadableFile> {
            if let Some(path) =
                self.native_file_dialog.as_mut().expect("native_file_dialog not instantiated").take_picked()
            {
                Some(ReadableFile { file: None, path })
            } else {
                None
            }
        }

        pub fn update(&mut self, ui: &mut egui::Ui) -> () {
            #[cfg(not(target_arch = "wasm32"))]
            self.native_file_dialog.as_mut().expect("native_file_dialog not instantiated").update(&ui);
        }
    }

    pub struct ReadableFile {
        path: PathBuf,
        file: Option<File>,
    }

    impl ReadableFile {
        pub fn path(&self) -> Option<PathBuf> {
            Some(self.path.clone())
        }
        pub fn name(&self) -> String {
            self.path.file_name().unwrap_or_default().to_string_lossy().to_string()
        }
    }

    impl Read for ReadableFile {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            if let Some(file) = &mut self.file {
                return file.read(buf);
            }
            let mut file = File::open(self.path.clone())?;
            let res = file.read(buf);
            self.file = Some(file);
            res
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod web {
    use std::io;
    use std::{io::Read, path::PathBuf};

    use eframe::wasm_bindgen::prelude::Closure;
    use log::info;
    use web_sys::js_sys::{ArrayBuffer, Uint8Array};
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

        pub fn take_picked(&mut self) -> Option<ReadableFile> {
            let file_input = self.html_file_input.as_mut().expect("html_file_input not instantiated");
            let js_file_obj = file_input.files()?.get(0);
            file_input.set_value(""); // TODO: this should return Some only once, and then switch back to None

            if let Some(js_file_obj) = js_file_obj {
                Some(ReadableFile { js_file_obj })
            } else {
                None
            }
        }

        pub fn update(&mut self, _ui: &mut egui::Ui) -> () {}
    }

    pub struct ReadableFile {
        js_file_obj: web_sys::File,
    }

    impl ReadableFile {
        pub fn path(&self) -> Option<PathBuf> {
            None
        }
        pub fn name(&self) -> String {
            self.js_file_obj.name()
        }
    }

    use web_sys::wasm_bindgen::JsCast;
    use web_sys::{Event, FileReader};
    impl Read for ReadableFile {
        fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
            let filereader = FileReader::new().unwrap().dyn_into::<FileReader>().unwrap();
            let onload = Closure::wrap(Box::new(move |event: Event| {
                let filereader = event.target().unwrap().dyn_into::<FileReader>().unwrap();
                let array_buffer: ArrayBuffer = filereader.result().unwrap().dyn_into::<ArrayBuffer>().unwrap();
                let uint8_array = Uint8Array::new(&array_buffer);
                let bytes_vec: Vec<u8> = uint8_array.to_vec();
                info!("file loaded: {:?} ({} bytes)", bytes_vec, bytes_vec.len());
            }) as Box<dyn FnMut(_)>);

            filereader.set_onloadend(Some(onload.as_ref().unchecked_ref()));
            filereader.read_as_array_buffer(&self.js_file_obj).expect("this");
            onload.forget();
            Err(io::Error::new(io::ErrorKind::WouldBlock, "testing"))
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub type CompatFileDialog = egui_dialog::CompatFileDialog;
#[cfg(not(target_arch = "wasm32"))]
pub type HtmlInputElement = egui_dialog::HtmlInputElement;
#[cfg(not(target_arch = "wasm32"))]
pub type ReadableFile = egui_dialog::ReadableFile;

#[cfg(target_arch = "wasm32")]
pub type HtmlInputElement = web::HtmlInputElement;
#[cfg(target_arch = "wasm32")]
pub type CompatFileDialog = web::CompatFileDialog;
#[cfg(target_arch = "wasm32")]
pub type ReadableFile = web::ReadableFile;
