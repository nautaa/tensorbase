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

use std::sync::Arc;
use std::{any::Any, pin::Pin};

use crate::memory_stream::MemoryStream;
use crate::serde::scheduler::PartitionLocation;

use async_trait::async_trait;
use datafusion::arrow::datatypes::SchemaRef;
use datafusion::physical_plan::{DisplayFormatType, ExecutionPlan, Partitioning};
use datafusion::{
    error::{DataFusionError, Result},
    physical_plan::RecordBatchStream,
};
use log::info;
use std::fmt::Formatter;

/// UnresolvedShuffleExec represents a dependency on the results of a ShuffleWriterExec node which hasn't computed yet.
///
/// An ExecutionPlan that contains an UnresolvedShuffleExec isn't ready for execution. The presence of this ExecutionPlan
/// is used as a signal so the scheduler knows it can't start computation until the dependent shuffle has completed.
#[derive(Debug, Clone)]
pub struct UnresolvedShuffleExec {
    // The query stage ids which needs to be computed
    pub stage_id: usize,

    // The schema this node will have once it is replaced with a ShuffleReaderExec
    pub schema: SchemaRef,

    // The partition count this node will have once it is replaced with a ShuffleReaderExec
    pub partition_count: usize,
}

impl UnresolvedShuffleExec {
    /// Create a new UnresolvedShuffleExec
    pub fn new(stage_id: usize, schema: SchemaRef, partition_count: usize) -> Self {
        Self {
            stage_id,
            schema,
            partition_count,
        }
    }
}

#[async_trait]
impl ExecutionPlan for UnresolvedShuffleExec {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn schema(&self) -> SchemaRef {
        self.schema.clone()
    }

    fn output_partitioning(&self) -> Partitioning {
        Partitioning::UnknownPartitioning(self.partition_count)
    }

    fn children(&self) -> Vec<Arc<dyn ExecutionPlan>> {
        vec![]
    }

    fn with_new_children(
        &self,
        _children: Vec<Arc<dyn ExecutionPlan>>,
    ) -> Result<Arc<dyn ExecutionPlan>> {
        Err(DataFusionError::Plan(
            "Ballista UnresolvedShuffleExec does not support with_new_children()"
                .to_owned(),
        ))
    }

    async fn execute(
        &self,
        _partition: usize,
    ) -> Result<Pin<Box<dyn RecordBatchStream + Send + Sync>>> {
        Err(DataFusionError::Plan(
            "Ballista UnresolvedShuffleExec does not support execution".to_owned(),
        ))
    }

    fn fmt_as(
        &self,
        t: DisplayFormatType,
        f: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        match t {
            DisplayFormatType::Default => {
                write!(f, "UnresolvedShuffleExec")
            }
        }
    }
}
