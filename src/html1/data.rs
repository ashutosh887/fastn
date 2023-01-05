pub struct DataGenerator<'a> {
    pub doc: &'a ftd::interpreter2::TDoc<'a>,
}

impl<'a> DataGenerator<'a> {
    pub(crate) fn new(doc: &'a ftd::interpreter2::TDoc<'a>) -> DataGenerator<'a> {
        DataGenerator { doc }
    }

    pub(crate) fn get_data(&self) -> ftd::html1::Result<ftd::Map<serde_json::Value>> {
        let mut d: ftd::Map<serde_json::Value> = Default::default();
        for (k, v) in self.doc.bag().iter() {
            if let ftd::interpreter2::Thing::Variable(ftd::interpreter2::Variable {
                value, ..
            }) = v
            {
                let value = value.clone().resolve(self.doc, value.line_number())?;
                if let Some(value) = get_value(self.doc, &value)? {
                    d.insert(ftd::html1::utils::js_reference_name(k), value);
                }
            }
        }
        return Ok(d);

        fn get_value(
            doc: &ftd::interpreter2::TDoc,
            value: &ftd::interpreter2::Value,
        ) -> ftd::html1::Result<Option<serde_json::Value>> {
            if let ftd::interpreter2::Value::List { data, .. } = value {
                let mut list_data = vec![];
                for val in data {
                    let value = match val {
                        ftd::interpreter2::PropertyValue::Value { value, .. } => value,
                        _ => continue, //todo
                    };
                    if let Some(val) = get_value(doc, value)? {
                        list_data.push(val);
                    }
                }
                return Ok(serde_json::to_value(&list_data).ok());
            }
            let value = value.inner();

            Ok(match value {
                None => Some(serde_json::Value::Null),
                Some(ftd::interpreter2::Value::Boolean { value }) => {
                    serde_json::to_value(value).ok()
                }
                Some(ftd::interpreter2::Value::Integer { value }) => {
                    serde_json::to_value(value).ok()
                }
                Some(ftd::interpreter2::Value::String { text: value, .. }) => {
                    serde_json::to_value(value).ok()
                }
                Some(ftd::interpreter2::Value::Record { fields, .. }) => {
                    let mut value_fields = ftd::Map::new();
                    for (k, v) in fields {
                        if let Some(value) =
                            get_value(doc, &v.clone().resolve(doc, v.line_number())?)?
                        {
                            value_fields.insert(k, value);
                        }
                    }
                    serde_json::to_value(value_fields).ok()
                }
                Some(ftd::interpreter2::Value::OrType {
                    value,
                    variant,
                    full_variant,
                    name,
                    ..
                }) => {
                    let value = get_value(doc, &value.clone().resolve(doc, value.line_number())?)?;
                    match value {
                        Some(value) if name.eq(ftd::interpreter2::FTD_LENGTH) => {
                            if let Ok(pattern) = ftd::executor::Length::set_value_from_variant(
                                variant.as_str(),
                                value.to_string().as_str(),
                                doc.name,
                                0,
                            ) {
                                serde_json::to_value(pattern).ok()
                            } else {
                                Some(value)
                            }
                        }
                        Some(value) if name.eq(ftd::interpreter2::FTD_FONT_SIZE) => {
                            if let Ok(pattern) = ftd::executor::FontSize::set_value_from_variant(
                                variant.as_str(),
                                value.to_string().as_str(),
                                doc.name,
                                0,
                            ) {
                                serde_json::to_value(pattern).ok()
                            } else {
                                Some(value)
                            }
                        }
                        Some(value)
                            if name.eq(ftd::interpreter2::FTD_RESIZING_FIXED)
                                && variant.ne(ftd::interpreter2::FTD_RESIZING_FIXED) =>
                        {
                            if let Ok(pattern) = ftd::executor::Resizing::set_value_from_variant(
                                variant.as_str(),
                                full_variant.as_str(),
                                doc.name,
                                value.to_string().as_str(),
                                0,
                            ) {
                                serde_json::to_value(pattern).ok()
                            } else {
                                Some(value)
                            }
                        }
                        Some(value) => Some(value),
                        None => None,
                    }
                }
                _ => None,
            })
        }
    }
}
