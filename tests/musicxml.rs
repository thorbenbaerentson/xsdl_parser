

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    fn get_file_path(file_name : &str) -> PathBuf {
        let mut enivronment = std::env::current_dir().unwrap();
        enivronment.push("tests");
        enivronment.push("sample_files");
        enivronment.push(file_name);

        PathBuf::from_str(enivronment.to_str().unwrap()).unwrap()
    }

    #[test]
    fn musicxml_xsd() {
        const FILE : &str = "musicxml.xsd";
        let path = get_file_path(FILE);

        println!("File path: {:?}", path.clone());
        assert!(std::fs::exists(path.clone()).is_ok());

        let schema = xsdl_parser::prelude::Schema::load(&path);
        match schema {
            Ok(_) => { },
            Err(e) => {
                panic!("Could not read example. Error: {}", e);
            },
        }
    }
}