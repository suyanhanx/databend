// Copyright 2021 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::marker::PhantomData;

use common_exception::Result;

use crate::prelude::*;

pub struct NumberSerializer<T: PrimitiveType> {
    t: PhantomData<T>,
}

impl<T: PrimitiveType> Default for NumberSerializer<T> {
    fn default() -> Self {
        Self {
            t: Default::default(),
        }
    }
}

impl<T: PrimitiveType> TypeSerializer for NumberSerializer<T> {
    fn serialize_value(&self, value: &DataValue) -> Result<String> {
        Ok(format!("{:?}", value))
    }

    fn serialize_column(&self, column: &ColumnRef) -> Result<Vec<String>> {
        let array: &PrimitiveColumn<T> = unsafe { Series::static_cast(column) };
        let result: Vec<String> = array
            .iter()
            .map(|x| {
                 format!("{}", x)
            })
            .collect();
        Ok(result)
    }
}
