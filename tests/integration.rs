use rtranslate::{translate, translate_vec};

#[test]
fn test_translate_single() {
    let res = translate("Hello", "auto", "vi");
    assert!(res.is_ok(), "Expected Ok, got {:?}", res);
}

#[test]
fn test_translate_vec_batch() {
    let phrases = ["Hi", "Bye"];
    let results = translate_vec(&phrases, "auto", "vi");
    assert_eq!(results.len(), 2);
}
