//! Dictionary object
use super::{BoxedObject, Object, RefValue};
use indexmap::IndexMap;
use macros::tokay_method;

// Alias for the inner dict
type InnerDict = IndexMap<String, RefValue>;

// Dict object type
#[derive(Debug, Clone, PartialEq)]
pub struct Dict {
    dict: InnerDict,
}

impl Object for Dict {
    fn severity(&self) -> u8 {
        20
    }

    fn name(&self) -> &'static str {
        "dict"
    }

    fn repr(&self) -> String {
        let mut ret = "(".to_string();

        for (key, value) in self.iter() {
            if ret.len() > 1 {
                ret.push_str(", ");
            }

            // todo: Put this into a utility function...
            if !key.chars().all(|ch| ch.is_alphabetic() || ch == '_') {
                ret.push('"');
                for ch in key.chars() {
                    match ch {
                        '\n' => ret.push_str("\\n"),
                        '\r' => ret.push_str("\\r"),
                        '\t' => ret.push_str("\\t"),
                        '"' => ret.push_str("\\\""),
                        _ => ret.push(ch),
                    }
                }
                ret.push('"');
            } else {
                ret.push_str(key);
            }

            ret.push_str(" => ");
            ret.push_str(&value.borrow().repr());
        }

        ret.push(')');
        ret
    }

    fn is_true(&self) -> bool {
        self.len() > 0
    }
}

impl Dict {
    pub fn new() -> Self {
        Self {
            dict: InnerDict::new(),
        }
    }

    tokay_method!("dict()", Ok(RefValue::from(Dict::new())));

    tokay_method!("dict_len(dict)", {
        let dict = dict.borrow();

        if let Some(dict) = dict.object::<Dict>() {
            Ok(RefValue::from(dict.len()))
        } else {
            Err(format!(
                "{} only accepts '{}' as parameter, not '{}'",
                __function,
                "dict",
                dict.name()
            ))
        }
    });

    tokay_method!("dict_update(dict, other)", {
        {
            let dict = &mut *dict.borrow_mut();
            let other = &*other.borrow();

            if let Some(dict) = dict.object_mut::<Dict>() {
                if let Some(other) = other.object::<Dict>() {
                    for (k, v) in other.iter() {
                        dict.insert(k.clone(), v.clone());
                    }
                } else {
                    return Err(format!(
                        "{} only accepts '{}' as second parameter, not '{}'",
                        __function,
                        dict.name(),
                        other.name()
                    ));
                }
            } else {
                return Err(format!(
                    "{} only accepts '{}' as first parameter, not '{}'",
                    __function,
                    "dict",
                    dict.name()
                ));
            }
        }

        Ok(dict)
    });

    /*
    fn get_index(&self, index: &Value) -> Result<RefValue, String> {
        let index = index.to_string();
        if let Some(value) = self.get(&index) {
            Ok(value.clone())
        } else {
            Err(format!("Key '{}' not found", index))
        }
    }

    fn set_index(&mut self, index: &Value, value: RefValue) -> Result<(), String> {
        let index = index.to_string();
        self.insert(index, value);
        Ok(())
    }
    */
}

impl PartialOrd for Dict {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!();
    }
}

impl std::ops::Deref for Dict {
    type Target = InnerDict;

    fn deref(&self) -> &Self::Target {
        &self.dict
    }
}

impl std::ops::DerefMut for Dict {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dict
    }
}

impl From<Dict> for RefValue {
    fn from(value: Dict) -> Self {
        RefValue::from(Box::new(value) as BoxedObject)
    }
}

#[test]
fn test_dict() {
    assert_eq!(
        crate::utils::compile_and_run("(b => 3, c => 1, a => 2)", ""),
        Ok(Some(crate::value!(["a" => 2, "b" => 3, "c" => 1])))
    );
}

#[test]
fn test_dict_len() {
    assert_eq!(
        crate::utils::compile_and_run("dict().len() (a => 1, b => 2).len()", ""),
        Ok(Some(crate::value!([(0 as usize), (2 as usize)])))
    );

    assert_eq!(
        crate::utils::compile_and_run("dict_len(\"Donkey\")", ""),
        Err("Line 1, column 1: dict_len() only accepts 'dict' as parameter, not 'str'".to_string())
    )
}

#[test]
fn test_dict_update() {
    assert_eq!(
        crate::utils::compile_and_run("d = (a => 1, b => 2); d.update((c => 3)); d", ""),
        Ok(Some(crate::value!(["a" => 1, "b" => 2, "c" => 3])))
    )
}
