extern crate tempdir;
extern crate tempfile;
extern crate cargo_gen_helpers;

mod test_create_file {
    use std::fs::File;
    use std::io::Read;
    use tempdir::TempDir;
    use cargo_gen_helpers::create_file;

    #[test]
    fn it_creates_a_file_with_supplied_content() {
        let tempdir = TempDir::new("cargo-gen-test").unwrap();
        let tempdir_path = tempdir.path();
        let tempfile_path = tempdir_path.join("file.rs");

        create_file(&tempfile_path, "pub fn foo() {}").unwrap();

        let mut content = String::new();
        File::open(tempfile_path).unwrap().read_to_string(&mut content).unwrap();
        assert_eq!("pub fn foo() {}", content);
        println!("{:?}", tempdir);
    }

    #[test]
    fn it_creates_the_parent_directories_that_do_not_exist() {
        let tempdir = TempDir::new("cargo-gen-test").unwrap();
        let tempdir_path = tempdir.path();
        let tempfile_path = tempdir_path.join("some/parent/directories/file.rs");

        create_file(&tempfile_path, "pub fn foo() {}").unwrap();

        let mut content = String::new();
        File::open(tempfile_path).unwrap().read_to_string(&mut content).unwrap();
        assert_eq!("pub fn foo() {}", content);
        println!("{:?}", tempdir);
    }
}

mod test_modify_file {
    use tempfile::NamedTempFile;
    use std::io::{Error, Read, Seek, SeekFrom, Write};
    use cargo_gen_helpers::modify_file;

    fn make_temp_file(content: &[u8]) -> Result<NamedTempFile, Error> {
        let mut tmp_file = NamedTempFile::new()?;
        tmp_file.write_all(content)?;
        Ok(tmp_file)
    }

    #[test]
    fn it_supplies_file_content_for_modification() {
        let tmp_file = make_temp_file(b"The content.").unwrap();

        let mut received_content = String::from("");
        modify_file(tmp_file.path(), |content| {
                received_content = content.to_owned();
                Ok(None)
            })
            .unwrap();
        assert_eq!("The content.", received_content);
    }

    #[test]
    fn it_supplies_file_content_by_file_path_string_too() {
        let tmp_file = make_temp_file(b"The content.").unwrap();

        let mut received_content = String::from("");
        modify_file(tmp_file.path().to_str().unwrap(), |content| {
                received_content = content.to_owned();
                Ok(None)
            })
            .unwrap();
        assert_eq!("The content.", received_content);
    }

    #[test]
    fn it_writes_new_content_to_file() {
        let mut tmp_file = make_temp_file(b"The content.").unwrap();

        modify_file(tmp_file.path(), |_| Ok(Some("New content.".to_string()))).unwrap();

        tmp_file.seek(SeekFrom::Start(0)).unwrap();
        let mut new_content = String::new();
        tmp_file.read_to_string(&mut new_content).unwrap();
        assert_eq!("New content.", new_content);
    }

    #[test]
    fn it_writes_modified_content_to_file() {
        let mut tmp_file = make_temp_file(b"The content.").unwrap();

        modify_file(tmp_file.path(),
                    |content| Ok(Some(content.replace("The", "New"))))
            .unwrap();

        tmp_file.seek(SeekFrom::Start(0)).unwrap();
        let mut new_content = String::new();
        tmp_file.read_to_string(&mut new_content).unwrap();
        assert_eq!("New content.", new_content);
    }
}
