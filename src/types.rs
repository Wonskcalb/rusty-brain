use std::fmt::{Debug, Display, Formatter, Result};
use std::fs::{create_dir_all, read, read_dir, write};

const PASTE_STORAGE: &str = "pastes";

pub struct PasteID(&'static str);

impl PasteID {
    pub fn new() -> PasteID {
        return PasteID("generated id");
    }

    pub fn val(self) -> &'static str {
        return self.0;
    }
}

impl Display for PasteID {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, FromForm)]
pub struct PasteData {
    seed_id: String,
    paste_name: String,
    paste_lang: String,
    paste_body: String,
}

impl PasteData {
    pub fn save(self) -> std::io::Result<()> {
        let file_path = format!(
            "{}/{}/{}.{}",
            PASTE_STORAGE, self.seed_id, self.paste_name, self.paste_lang
        );

        create_dir_all(format!("{}/{}", PASTE_STORAGE, self.seed_id))?;
        write(file_path, self.paste_body)?;

        Ok(())
    }

    // pub fn find(paste_id: String) -> PasteData {
    //     let dir_content = read_dir(format!("{}/{}", PASTE_STORAGE, paste_id))?;

    //     for entry in dir_content {
    //         let dir = entry?;
    //         println!("{:?}", dir.path());
    //     }

    //     // let file_name = dir_content.next();
    //     // let content = read(file_name.path());

    //     PasteData {
    //         paste_id: paste_id,
    //         paste_name: "Foo",
    //         paste_lang: String::from"rs",
    //         paste_body: String::from("This is a test"),
    //     }
    // }
}
