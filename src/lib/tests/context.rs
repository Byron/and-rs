extern crate anders;

mod context {
    use anders::{Context, ContextVerificationError};
    use std::io::Cursor;

    #[test]
    fn it_does_not_loose_information_during_serde() {
        let ctx = Context {
            application_name: "name".to_owned(),
            package_path: "package".to_owned(),
            target: "target".to_owned(),
        };

        assert_eq!(ctx,
        Context::deserialize(&mut Cursor::new(ctx.serialize())).unwrap());
    }

    fn ctx_from_target(name: &str) -> Context {
        Context {
            target: name.to_owned(),
            application_name: "name".to_owned(),
            ..Default::default()
        }
    }

    #[test]
    fn it_likes_latin_characters_and_dashes_in_target() {
        let name = "android-25";
        assert_eq!(ctx_from_target(name).verify(), Ok(()));
    }

    #[test]
    fn it_rejects_non_latin_literals_in_target() {
        let name = "$1hi!";
        assert_eq!(ctx_from_target(name).verify(),
        Err(ContextVerificationError::InvalidTargetName(name.to_owned())));
    }

    #[test]
    fn it_rejects_spaces_in_target() {
        let name = "hello android";
        assert_eq!(ctx_from_target(name).verify(),
        Err(ContextVerificationError::InvalidTargetName(name.to_owned())));
    }

    fn ctx_from_project(name: &str) -> Context {
        Context {
            application_name: name.to_owned(),
            target: "target".to_owned(),
            ..Default::default()
        }
    }

    #[test]
    fn it_likes_latin_characters_in_project() {
        let name = "5HelloWorld123";
        assert_eq!(ctx_from_project(name).verify(), Ok(()));
    }

    #[test]
    fn it_rejects_non_latin_literals_in_project() {
        let name = "$1hi!";
        assert_eq!(ctx_from_project(name).verify(),
        Err(ContextVerificationError::InvalidProjectName(name.to_owned())));
    }

    #[test]
    fn it_rejects_dashes_in_project() {
        let name = "Hello-World";
        assert_eq!(ctx_from_project(name).verify(),
        Err(ContextVerificationError::InvalidProjectName(name.to_owned())));
    }
}
