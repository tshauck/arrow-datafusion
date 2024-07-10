// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use std::fmt::Debug;

use arrow::datatypes::{DataType, Field};

use datafusion_common::Result;
use datafusion_expr::function::StateFieldsArgs;
use datafusion_expr::ReversedUDAF;
use datafusion_expr::{
    function::AccumulatorArgs, utils::format_state_name, Accumulator, AggregateUDFImpl,
    GroupsAccumulator, Signature, Volatility,
};

use crate::count::{CountAccumulator, CountGroupsAccumulator};

make_udaf_expr_and_func!(
    CountStar,
    count_star,
    expr,
    "Count the number of values",
    count_star_udaf
);

pub struct CountStar {
    signature: Signature,
}

impl Debug for CountStar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CountStar")
            .field("name", &self.name())
            .field("signature", &self.signature)
            .finish()
    }
}

impl Default for CountStar {
    fn default() -> Self {
        Self::new()
    }
}

impl CountStar {
    pub fn new() -> Self {
        Self {
            signature: Signature::any(0, Volatility::Immutable),
        }
    }
}

impl AggregateUDFImpl for CountStar {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn name(&self) -> &str {
        "count_star"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType> {
        Ok(DataType::Int64)
    }

    fn state_fields(&self, args: StateFieldsArgs) -> Result<Vec<Field>> {
        Ok(vec![Field::new(
            format_state_name(args.name, "count"),
            DataType::Int64,
            true,
        )])
    }

    fn accumulator(&self, _acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> {
        return Ok(Box::new(CountAccumulator::new()));
    }

    fn aliases(&self) -> &[String] {
        &[]
    }

    fn groups_accumulator_supported(&self, args: AccumulatorArgs) -> bool {
        if args.is_distinct {
            return false;
        }
        args.input_exprs.len() == 1
    }

    fn create_groups_accumulator(
        &self,
        _args: AccumulatorArgs,
    ) -> Result<Box<dyn GroupsAccumulator>> {
        // instantiate specialized accumulator
        Ok(Box::new(CountGroupsAccumulator::new()))
    }

    fn reverse_expr(&self) -> ReversedUDAF {
        ReversedUDAF::Identical
    }
}
