use handlebars::Handlebars;
use log::debug;
use std::fs::File;
use std::io::Read;

use crate::config::*;
use crate::error::*;

pub fn init_templates() -> BlogResult<Handlebars> {
    let mut reg = Handlebars::new();
    load_template(&mut reg, "post")?;
    load_template(&mut reg, "list")?;
    Ok(reg)
}

fn load_template(reg: &mut Handlebars, name: impl ToString) -> BlogResult<()> {
    let template_text =
        read_file_to_string(format!("{}/templates/{}.html", get_doc_path(), name.to_string()))?;
    reg.register_template_string(&name.to_string(), &template_text.to_string())?;
    Ok(())
}

pub fn read_file_to_string(path: String) -> BlogResult<String> {
    debug!("Opening file '{}'", path);
    let mut file_content = String::new();
    File::open(path)?.read_to_string(&mut file_content)?;
    Ok(file_content)
}
