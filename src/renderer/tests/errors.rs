use std::collections::BTreeMap;
use context::Context;
use errors::Result;
use tera::Tera;

#[test]
fn error_location_basic() {
    let mut tera = Tera::default();
    tera.add_raw_templates(vec![("tpl", "{{ 1 + true }}")])
        .unwrap();

    let result = tera.render("tpl", &Context::new());

    assert_eq!(
        result.unwrap_err().iter().nth(0).unwrap().description(),
        "Failed to render \'tpl\'"
    );
}

#[test]
fn error_location_inside_macro() {
    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        (
            "macros",
            "{% macro hello()%}{{ 1 + true }}{% endmacro hello %}",
        ),
        (
            "tpl",
            "{% import \"macros\" as macros %}{{ macro::hello() }}",
        ),
    ]).unwrap();

    let result = tera.render("tpl", &Context::new());

    assert_eq!(
        result.unwrap_err().iter().nth(0).unwrap().description(),
        "Failed to render \'tpl\': error while rendering a macro from the `macro` namespace"
    );
}

#[test]
fn error_location_base_template() {
    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        (
            "parent",
            "Hello {{ greeting + 1}} {% block bob %}{% endblock bob %}",
        ),
        (
            "child",
            "{% extends \"parent\" %}{% block bob %}Hey{% endblock bob %}",
        ),
    ]).unwrap();

    let result = tera.render("child", &Context::new());

    assert_eq!(
        result.unwrap_err().iter().nth(0).unwrap().description(),
        "Failed to render \'child\' (error happened in 'parent')."
    );
}

#[test]
fn error_location_in_parent_block() {
    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        (
            "parent",
            "Hello {{ greeting }} {% block bob %}{{ 1 + true }}{% endblock bob %}",
        ),
        (
            "child",
            "{% extends \"parent\" %}{% block bob %}{{ super() }}Hey{% endblock bob %}",
        ),
    ]).unwrap();

    let result = tera.render("child", &Context::new());

    assert_eq!(
        result.unwrap_err().iter().nth(0).unwrap().description(),
        "Failed to render \'child\' (error happened in 'parent')."
    );
}

#[test]
fn error_location_in_parent_in_macro() {
    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("macros", "{% macro hello()%}{{ 1 + true }}{% endmacro hello %}"),
        ("parent", "{% import \"macros\" as macros %}{{ macro::hello() }}{% block bob %}{% endblock bob %}"),
        ("child", "{% extends \"parent\" %}{% block bob %}{{ super() }}Hey{% endblock bob %}"),
    ]).unwrap();

    let result = tera.render("child", &Context::new());

    assert_eq!(
        result.unwrap_err().iter().nth(0).unwrap().description(),
        "Failed to render \'child\': error while rendering a macro from the `macro` namespace (error happened in \'parent\')."
    );
}

#[test]
fn error_out_of_range_index() {
    let mut tera = Tera::default();
    tera.add_raw_templates(vec![("tpl", "{{ arr[10] }}")])
        .unwrap();
    let mut context = Context::new();
    context.add("arr", &[1, 2, 3]);

    let result = tera.render("tpl", &Context::new());

    assert_eq!(
        result.unwrap_err().iter().nth(1).unwrap().description(),
        "Variable `arr[10]` not found in context while rendering \'tpl\'"
    );
}

#[test]
fn error_unknown_index_variable() {
    let mut tera = Tera::default();
    tera.add_raw_templates(vec![("tpl", "{{ arr[a] }}")])
        .unwrap();
    let mut context = Context::new();
    context.add("arr", &[1, 2, 3]);

    let result = tera.render("tpl", &Context::new());

    assert_eq!(
        result.unwrap_err().iter().nth(1).unwrap().description(),
        "Variable arr[a] can not be evaluated because: Variable `a` not found in context while rendering \'tpl\'"
    );
}

#[test]
fn error_invalid_type_index_variable() {
    let mut tera = Tera::default();
    tera.add_raw_templates(vec![("tpl", "{{ arr[a] }}")])
        .unwrap();

    let mut context = Context::new();
    context.add("arr", &[1, 2, 3]);
    context.add("a", &true);

    let result = tera.render("tpl", &context);

    assert_eq!(
        result.unwrap_err().iter().nth(1).unwrap().description(),
        "Only variables evaluating to String or Number can be used as index (`a` of `arr[a]`)"
    );
}
