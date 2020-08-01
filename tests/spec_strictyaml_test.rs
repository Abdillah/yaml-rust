#![cfg(feature = "strictyaml")]
extern crate yaml_rust;

mod strictyamltest {
    use yaml_rust::{Yaml, YamlLoader};
    use yaml_rust::yaml::Hash;

    const ERR_FLOW_DISALLOWED: &'static str = "flow disallowed";
    const ERR_OBJECT_DISALLOWED: &'static str = "object disallowed";
    const ERR_DUPLICATEKEY_DISALLOWED: &'static str = "duplicate key disallowed";
    const ERR_EXPLICITTAG_DISALLOWED: &'static str = "explicit tag disallowed";
    const ERR_REF_DISALLOWED: &'static str = "ref / anchor disallowed";

    fn build_hash(kv_arr: Vec<[Yaml; 2]>) -> Yaml {
        let mut hash = Hash::new();
        for c in kv_arr {
            hash.insert(c.get(0).unwrap().clone(), c.get(1).unwrap().clone());
        }
        Yaml::Hash(hash)
    }

    #[test]
    fn test_implicit_typings() {
        let fixtures: Vec<(&'static str, Yaml, Option<String>)> = vec!(
            // (Yaml string, Yaml result, Scanner error)
            (
                "x: yes\ny: null\n",
                build_hash(vec!(
                    [ Yaml::String("x".to_string()), Yaml::String("yes".to_string()) ],
                    [ Yaml::String("y".to_string()), Yaml::Null ],
                )),
                None,
            ),
            (
                "x: !!int 5\n",
                build_hash(vec!(
                    [ Yaml::String("x".to_string()), Yaml::BadValue ],
                )),
                Some(ERR_EXPLICITTAG_DISALLOWED.to_string()),
            ),
        );

        for (s, y, exp_err) in fixtures.iter() {
            let y = y.clone();
            if let Some(err_text) = exp_err.clone() {
                if let Err(err) = YamlLoader::load_from_str(s) {
                    assert!(err.to_string().starts_with(&err_text));
                } else {
                    panic!("Must return error: '{}'", err_text);
                }
            } else {
                let doc = YamlLoader::load_from_str(s).unwrap().pop().unwrap();
                assert_eq!(doc, y);
            }
        }
    }

    #[test]
    fn test_disallowed_flow_sequence() {
        let fixtures: Vec<(&'static str, Yaml, Option<String>)> = vec!(
            // (Yaml string, Yaml result, Scanner error)
            (
                "x: [ 1, 2, 3 ]\n",
                build_hash(vec!(
                    [ Yaml::String("x".to_string()), Yaml::BadValue ],
                )),
                Some(ERR_FLOW_DISALLOWED.to_string()),
            ),
        );

        for (s, y, exp_err) in fixtures.iter() {
            let y = y.clone();
            if let Some(err_text) = exp_err.clone() {
                if let Err(err) = YamlLoader::load_from_str(s) {
                    assert!(err.to_string().starts_with(&err_text));
                } else {
                    panic!("Must return error: '{}'", err_text);
                }
            } else {
                let doc = YamlLoader::load_from_str(s).unwrap().pop().unwrap();
                assert_eq!(doc, y);
            }
        }
    }
}