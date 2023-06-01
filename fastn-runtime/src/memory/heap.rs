pub type Heap<T> = slotmap::SlotMap<fastn_runtime::PointerKey, HeapData<T>>;

/// For every ftd value we have one such entry
#[derive(Debug)]
pub struct HeapData<T> {
    /// The inner value being stored in ftd
    pub value: HeapValue<T>,
    /// the list of values that depend on this, eg if we add x to a list l, we also do a
    /// x.dependents.add(l)
    pub dependents: Vec<fastn_runtime::memory::Pointer>,
    /// whenever a dom node is added or deleted, it is added or removed from this list.
    pub ui_properties: Vec<fastn_runtime::memory::DynamicProperty>,
}

/// This is the data we store in the heap for any value.
#[derive(Debug, Eq, PartialEq)]
pub enum HeapValue<T> {
    Value(T),

    /// If a value is defined in terms of a function, we store the last computed value and the
    /// closure. We cached the last computed value so if the data is not changing we do not have
    /// to re-compute the closure.
    ///
    /// -- integer x: 10 (stored as HeapValue::Value(10))
    /// -- integer y: 20 (stored as HeapValue::Value(10))
    /// -- integer z = { x + y } (stored as HeapValue::Formula { cached_value: 30, closure: 1v2 }
    Formula {
        cached_value: T,
        closure: fastn_runtime::ClosurePointer,
    },
}

impl<T> HeapData<T> {
    pub(crate) fn new(value: HeapValue<T>) -> HeapData<T> {
        HeapData {
            value,
            dependents: vec![],
            ui_properties: vec![],
        }
    }
}

impl<T> HeapValue<T> {
    pub(crate) fn mut_value(&mut self) -> &mut T {
        match self {
            HeapValue::Value(v) => v,
            _ => unimplemented!(),
        }
    }
    pub(crate) fn value(&self) -> &T {
        match self {
            HeapValue::Value(v) => v,
            _ => unimplemented!(),
        }
    }
    pub(crate) fn set_value(&mut self, v: T) {
        match self {
            HeapValue::Value(s) => *s = v,
            _ => unimplemented!(),
        }
    }
}

impl<T> HeapValue<T> {
    pub(crate) fn new(value: T) -> HeapValue<T> {
        HeapValue::Value(value)
    }

    pub(crate) fn new_with_formula(
        cached_value: T,
        closure: fastn_runtime::ClosurePointer,
    ) -> HeapValue<T> {
        HeapValue::Formula {
            cached_value,
            closure,
        }
    }

    pub(crate) fn into_heap_data(self) -> HeapData<T> {
        HeapData::new(self)
    }
}