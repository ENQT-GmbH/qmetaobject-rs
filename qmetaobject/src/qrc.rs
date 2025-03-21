/* Copyright (C) 2018 Olivier Goffart <ogoffart@woboq.com>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
associated documentation files (the "Software"), to deal in the Software without restriction,
including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense,
and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial
portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES
OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

cpp! {{
Q_CORE_EXPORT bool qRegisterResourceData(int, const unsigned char *,
                                         const unsigned char *, const unsigned char *);
}}

/// Internal function used from qrc procedural macro.
/// Unsafe because it can crash if the data structure are not proper.
#[doc(hidden)]
pub unsafe fn register_resource_data(
    version: i32,
    tree: &'static [u8],
    names: &'static [u8],
    payload: &'static [u8],
) {
    let tree_ptr = tree.as_ptr();
    let names_ptr = names.as_ptr();
    let payload_ptr = payload.as_ptr();
    cpp!([version as "int", tree_ptr as "const unsigned char*", names_ptr as "const unsigned char*", payload_ptr as "const unsigned char*"] {
        qRegisterResourceData(version, tree_ptr, names_ptr, payload_ptr);
    });
}
