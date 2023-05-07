use ryol::prelude::*;

#[test]
fn structure_definition_syntax() {
    assert!(eval("(def-struct)").is_err());
    assert!(eval("(def-struct none)").is_ok());
    assert!(eval("(def-struct vec1d x)").is_ok());
    assert!(eval("(def-struct vec2d x y)").is_ok());
    assert!(eval("(def-struct vec3d x y z)").is_ok());
}

#[test]
fn structure_standard_usage() {
    let mut vec2d_template = StructureTemplate::new();
    vec2d_template.add_member(&"x".to_string()).unwrap();
    vec2d_template.add_member(&"y".to_string()).unwrap();
    let vec2d = StructureInstance::from_template(&vec2d_template);

    assert_eq!(eval("(def-struct vec2d x y)").unwrap(), Value::default());
    assert_eq!(
        eval("(def-struct vec2d x y) (vec2d)").unwrap(),
        Value::Structure(vec2d)
    );

    assert!(
        eval("(def-struct vec2d x y) (set v (vec2d)) (set-member v z 2)").is_err(),
    );

    assert_eq!(
        eval("(def-struct vec2d x y) (set v (vec2d)) (set-member v x 2) (get-member v x)").unwrap(),
        Value::Integer(2)
    );
    assert_eq!(
        eval("(def-struct vec2d x y) (set v (vec2d)) (set-member v y 3) (get-member v y)").unwrap(),
        Value::Integer(3)
    );
    assert_eq!(
        eval("(def-struct vec2d x y) (set v (vec2d)) (set-member v x 2) (set-member v y 3) (get-member v x)").unwrap(),
        Value::Integer(2)
    );
    assert_eq!(
        eval("(def-struct vec2d x y) (set v (vec2d)) (set-member v x 2) (set-member v y 3) (get-member v y)").unwrap(),
        Value::Integer(3)
    );
}
