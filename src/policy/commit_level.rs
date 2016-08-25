// Copyright 2012-2016 Aerospike, Inc.
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
//

// CommitLevel determines how to handle record writes based on record generation.
#[derive(Debug,PartialEq)]
pub enum CommitLevel {
	// CommitAll indicates the server should wait until successfully committing master and all replicas.
	CommitAll = 0,

	// CommitMaster indicates the server should wait until successfully committing master only.
	CommitMaster
}

impl Default for CommitLevel {
    fn default() -> CommitLevel {
        CommitLevel::CommitAll
    }
}