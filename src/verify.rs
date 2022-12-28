pub fn verify_content(data: Option<String>) -> Result<(),String> {
    if let Some(v) = data {
        return if v.len() < 10 { Err(format!("Content length must be more than 10 chars")) } else { Ok(()) };
    }
    Err(format!("Content param required"))
}
