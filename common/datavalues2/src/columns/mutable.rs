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

use std::any::Any;
use std::sync::Arc;

use common_arrow::arrow::bitmap::MutableBitmap;

use crate::types::DataTypePtr;
use crate::Column;
use crate::ColumnRef;

pub trait MutableColumn<Item, ColumnImpl: Column + 'static> {
    fn data_type(&self) -> DataTypePtr;
    fn with_capacity(capacity: usize) -> Self;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;

    fn as_column(&mut self) -> ColumnRef {
        Arc::new(self.finish())
    }

    fn finish(&mut self) -> ColumnImpl;
    fn append(&mut self, item: Item);
    fn append_default(&mut self);

    fn validity(&self) -> Option<&MutableBitmap> {
        None
    }

    fn shrink_to_fit(&mut self);
}
