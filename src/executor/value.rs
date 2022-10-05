#[derive(serde::Deserialize, Debug, PartialEq, Default, Clone, serde::Serialize)]
pub struct Value<T> {
    pub value: T,
    pub properties: Vec<ftd::interpreter2::Property>,
}

impl<T> Value<T> {
    pub fn new(value: T, properties: Vec<ftd::interpreter2::Property>) -> Value<T> {
        Value { value, properties }
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Value<U> {
        Value {
            value: f(self.value),
            properties: self.properties,
        }
    }
}

pub(crate) fn get_value_from_properties_using_key_and_arguments(
    key: &str,
    properties: &[ftd::interpreter2::Property],
    arguments: &[ftd::interpreter2::Argument],
    doc: &ftd::executor::TDoc,
    line_number: usize,
) -> ftd::executor::Result<ftd::executor::Value<Option<ftd::interpreter2::Value>>> {
    let argument =
        arguments
            .iter()
            .find(|v| v.name.eq(key))
            .ok_or(ftd::executor::Error::ParseError {
                message: format!("Cannot find `{}` argument", key),
                doc_id: doc.name.to_string(),
                line_number,
            })?;

    let sources = argument.to_sources();
    let ftd::executor::Value { properties, value } =
        find_value_by_source(sources.as_slice(), properties, doc, line_number)?;
    let expected_kind = value.as_ref().map(|v| v.kind());
    if !expected_kind
        .as_ref()
        .map_or(true, |v| v.is_same_as(&argument.kind.kind))
    {
        return ftd::executor::utils::parse_error(
            format!(
                "Expected kind {:?}, found: `{:?}`",
                expected_kind, argument.kind.kind
            ),
            doc.name,
            line_number,
        );
    }

    Ok(ftd::executor::Value::new(value, properties))
}

pub(crate) fn find_value_by_source(
    source: &[ftd::interpreter2::PropertySource],
    properties: &[ftd::interpreter2::Property],
    doc: &ftd::executor::TDoc,
    line_number: usize,
) -> ftd::executor::Result<ftd::executor::Value<Option<ftd::interpreter2::Value>>> {
    use itertools::Itertools;

    let properties = properties
        .iter()
        .filter(|v| source.iter().any(|s| v.source.is_equal(s)))
        .map(ToOwned::to_owned)
        .collect_vec();
    if properties.is_empty() {
        return ftd::executor::utils::parse_error(
            "Cannot find any property for given source",
            doc.name,
            line_number,
        );
    }

    ftd::executor::utils::validate_properties(properties.as_slice(), doc.name)?;

    let mut value = None;
    for p in properties.iter() {
        if let Some(v) = p.resolve(&doc.itdoc())? {
            value = Some(v);
            if p.condition.is_some() {
                break;
            }
        }
    }

    Ok(ftd::executor::Value::new(value, properties))
}

pub fn string(
    key: &str,
    properties: &[ftd::interpreter2::Property],
    arguments: &[ftd::interpreter2::Argument],
    doc: &ftd::executor::TDoc,
    line_number: usize,
) -> ftd::executor::Result<ftd::executor::Value<String>> {
    let value = get_value_from_properties_using_key_and_arguments(
        key,
        properties,
        arguments,
        doc,
        line_number,
    )?;

    match value.value.and_then(|v| v.inner()) {
        Some(ftd::interpreter2::Value::String { text }) => {
            Ok(ftd::executor::Value::new(text, value.properties))
        }
        t => ftd::executor::utils::parse_error(
            format!("Expected value of type string, found: {:?}", t),
            doc.name,
            line_number,
        ),
    }
}

pub fn optional_i64(
    key: &str,
    properties: &[ftd::interpreter2::Property],
    arguments: &[ftd::interpreter2::Argument],
    doc: &ftd::executor::TDoc,
    line_number: usize,
) -> ftd::executor::Result<ftd::executor::Value<Option<i64>>> {
    let value = get_value_from_properties_using_key_and_arguments(
        key,
        properties,
        arguments,
        doc,
        line_number,
    )?;

    match value.value.and_then(|v| v.inner()) {
        Some(ftd::interpreter2::Value::Integer { value: v }) => {
            Ok(ftd::executor::Value::new(Some(v), value.properties))
        }
        None => Ok(ftd::executor::Value::new(None, value.properties)),
        t => ftd::executor::utils::parse_error(
            format!("Expected value of type string, found: {:?}", t),
            doc.name,
            line_number,
        ),
    }
}