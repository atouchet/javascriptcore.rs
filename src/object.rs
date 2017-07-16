// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ops::Deref;
use std::ptr;
use super::{JSObject, JSString, JSValue};
use sys;

impl JSObject {
    /// Gets an iterator over the names of an object's enumerable properties.
    ///
    /// ```
    /// # use javascriptcore::{JSObject, JSString};
    /// # fn get_property_names(obj: JSObject) {
    /// let names: Vec<JSString> = obj.property_names().collect();
    /// # }
    /// ```
    pub fn property_names(&self) -> JSObjectPropertyNameIter {
        JSObjectPropertyNameIter {
            raw: unsafe { sys::JSObjectCopyPropertyNames(self.value.ctx, self.raw) },
            idx: 0,
        }
    }

    /// Tests whether an object has a given property.
    ///
    /// * `name`: A value that can be converted to a `JSString` containing
    ///   the property's name.
    ///
    /// Returns `true` if the object has a property whose name matches
    /// `name`, otherwise `false`.
    ///
    /// ```
    /// # use javascriptcore::JSObject;
    /// # fn has_property(obj: JSObject) {
    /// if obj.has_property("id") {
    ///     // ...
    /// }
    /// # }
    /// ```
    pub fn has_property<S>(&self, name: S) -> bool
    where
        S: Into<JSString>,
    {
        unsafe { sys::JSObjectHasProperty(self.value.ctx, self.raw, name.into().raw) }
    }

    /// Gets a property from an object.
    ///
    /// * `name`: A value that can be converted to a `JSString` containing
    ///   the property's name.
    ///
    /// Returns the property's value if object has the property, otherwise
    /// the undefined value.
    pub fn get_property<S>(&self, name: S) -> JSValue
    where
        S: Into<JSString>,
    {
        let mut e: sys::JSValueRef = ptr::null_mut();
        let v =
            unsafe { sys::JSObjectGetProperty(self.value.ctx, self.raw, name.into().raw, &mut e) };
        JSValue {
            raw: v,
            ctx: self.value.ctx,
        }
    }

    /// Gets a property from an object by numeric index.
    ///
    /// * `index`: An integer value that is the property's name.
    ///
    /// Returns the property's value if object has the property,
    /// otherwise the undefined value.
    ///
    /// Calling `get_property_at_index` is equivalent to calling
    /// `get_property` with a string containing `index`,
    /// but `get_property_at_index` provides optimized access to
    /// numeric properties.
    pub fn get_property_at_index(&self, index: u32) -> JSValue {
        let mut e: sys::JSValueRef = ptr::null_mut();
        let v = unsafe { sys::JSObjectGetPropertyAtIndex(self.value.ctx, self.raw, index, &mut e) };
        JSValue {
            raw: v,
            ctx: self.value.ctx,
        }
    }
}

impl Deref for JSObject {
    type Target = JSValue;

    fn deref(&self) -> &JSValue {
        &self.value
    }
}

pub struct JSObjectPropertyNameIter {
    raw: sys::JSPropertyNameArrayRef,
    idx: usize,
}

impl Iterator for JSObjectPropertyNameIter {
    type Item = JSString;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < unsafe { sys::JSPropertyNameArrayGetCount(self.raw) } {
            let name = unsafe { sys::JSPropertyNameArrayGetNameAtIndex(self.raw, self.idx) };
            self.idx += 1;
            Some(JSString { raw: name })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = unsafe { sys::JSPropertyNameArrayGetCount(self.raw) };
        (sz - self.idx, Some(sz))
    }
}

#[cfg(test)]
mod tests {
    use super::super::{JSContext, JSValue};

    #[test]
    fn can_has_property() {
        let ctx = JSContext::default();
        let v = JSValue::new_from_json(&ctx, "{\"id\": 123}").expect("value");
        let o = v.as_object().expect("object");
        assert!(o.has_property("id"));
        assert!(o.has_property("no-such-value") == false);
    }

    #[test]
    fn can_get_property() {
        let ctx = JSContext::default();
        let v = JSValue::new_from_json(&ctx, "{\"id\": 123}").expect("value");
        let o = v.as_object().expect("object");
        assert!(o.get_property("id").is_number());
        assert!(o.get_property("no-such-value").is_undefined());
    }

    #[test]
    fn can_get_property_at_index() {
        let ctx = JSContext::default();
        let v = JSValue::new_from_json(&ctx, "[3, true, \"abc\"]").expect("value");
        let o = v.as_object().expect("object");
        assert!(o.get_property_at_index(0).is_number());
        assert!(o.get_property_at_index(1).is_boolean());
        assert!(o.get_property_at_index(2).is_string());
        assert!(o.get_property_at_index(5).is_undefined());
    }

    #[test]
    fn can_get_property_names() {
        let ctx = JSContext::default();
        let v = JSValue::new_from_json(&ctx, "{\"id\": 123}").expect("value");
        let o = v.as_object().expect("object");
        let names = o.property_names().collect::<Vec<_>>();
        assert_eq!(names.len(), 1);
        assert_eq!(names[0], "id".into());
    }

    #[test]
    fn can_use_as_jsvalue_via_deref() {
        let ctx = JSContext::default();
        let v = JSValue::new_from_json(&ctx, "{\"id\": 123}").expect("value");
        let o = v.as_object().expect("object");
        assert!(v.is_object());
        assert!(o.is_object());
    }
}
