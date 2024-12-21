#[cfg(test)]
mod tests {
    #[test]
    fn dawproject_xsd() {
        const URL: &str =
            "https://raw.githubusercontent.com/bitwig/dawproject/refs/heads/main/Project.xsd";

        let schema = xsdl_parser::prelude::Schema::download(URL);
        match schema {
            Ok(_) => {}
            Err(e) => {
                panic!("Could finish test. Error: {}", e);
            }
        }
    }
}
