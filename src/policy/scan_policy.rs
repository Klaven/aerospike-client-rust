// Copyright 2015-2017 Aerospike, Inc.
//
// Portions may be licensed to Aerospike, Inc. under one or more contributor
// license agreements.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not
// use this file except in compliance with the License. You may obtain a copy of
// the License at http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations under
// the License.

use policy::{BasePolicy, PolicyLike};

/// ScanPolicy encapsulates optional parameters used in scan operations.
#[derive(Debug,Clone)]
pub struct ScanPolicy {

    /// Base policy instance
    pub base_policy: BasePolicy,

    /// Percent of data to scan. Valid integer range is 1 to 100. Default is 100.
    pub scan_percent: u8,

    /// Maximum number of concurrent requests to server nodes at any point in time. If there are 16
    /// nodes in the cluster and `max_concurrent_nodes` is 8, then scan requests will be made to 8
    /// nodes in parallel. When a scan completes, a new scan request will be issued until all 16
    /// nodes have been scanned. Default (0) is to issue requests to all server nodes in parallel.
    pub max_concurrent_nodes: usize,

    /// Number of records to place in queue before blocking. Records received from multiple server
    /// nodes will be placed in a queue. A separate thread consumes these records in parallel. If
    /// the queue is full, the producer threads will block until records are consumed.
    pub record_queue_size: usize,

    /// Indicates if bin data is retrieved. If false, only record digests are retrieved.
    pub include_bin_data: bool,

    /// Terminate scan if cluster is in fluctuating state.
    pub fail_on_cluster_change: bool,
}


impl ScanPolicy {

    /// Create a new scan policy instance with default parameters.
    pub fn new() -> Self {
        ScanPolicy::default()
    }
}

impl Default for ScanPolicy {
    fn default() -> ScanPolicy {
        ScanPolicy {
            base_policy: BasePolicy::default(),
            scan_percent: 100,
            max_concurrent_nodes: 0,
            record_queue_size: 1024,
            include_bin_data: true,
            fail_on_cluster_change: true,
        }
    }
}

impl PolicyLike for ScanPolicy {
    fn base(&self) -> &BasePolicy {
        &self.base_policy
    }
}
