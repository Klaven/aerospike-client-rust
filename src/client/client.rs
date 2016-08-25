// Copyright 2015-2016 Aerospike, Inc.
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

use log::LogLevel;
use env_logger;

use std::collections::{HashMap, VecDeque};
use std::sync::{RwLock, Arc, Mutex};
use std::vec::Vec;
use std::thread;
use std::time::{Instant, Duration};

use internal::wait_group::WaitGroup;
use net::Host;
use cluster::{Cluster, Node};
use common::operation;
use common::{Key, Record, Bin};
use command::read_command::{ReadCommand};
use command::write_command::{WriteCommand};
use command::delete_command::{DeleteCommand};
use command::touch_command::{TouchCommand};
use command::exists_command::{ExistsCommand};
use command::read_header_command::{ReadHeaderCommand};
use command::operate_command::{OperateCommand};
use value::{Value};

use policy::{ClientPolicy, ReadPolicy, WritePolicy};
use error::{AerospikeResult};


// Client encapsulates an Aerospike cluster.
// All database operations are available against this object.
pub struct Client {
    pub cluster: Arc<Cluster>,
}

impl Client {
    pub fn new(policy: &ClientPolicy, hosts: &[Host]) -> AerospikeResult<Self> {
        let cluster = try!(Cluster::new(policy.clone(), hosts));

        Ok(Client {
            cluster: cluster,
        })
    }

    pub fn close(&self) -> AerospikeResult<()> {
    	self.cluster.close()
    }

    pub fn is_connected(&self) -> bool {
    	self.cluster.is_connected()
    }

    pub fn nodes(&self) -> AerospikeResult<Vec<Arc<Node>>> {
    	Ok(self.cluster.nodes())
    }

    pub fn get<'a, 'b>(&'a self,
                         policy: &'a ReadPolicy,
                         key: &'a Key<'a>,
                         bin_names: Option<&'a [&'b str]>)
                         -> AerospikeResult<Arc<Record>> {
        let mut command = try!(ReadCommand::new(policy, self.cluster.clone(), key, bin_names));
        try!(command.execute());
        Ok(command.record.as_ref().unwrap().clone())
    }

    pub fn get_header<'a, 'b>(&'a self,
                         policy: &'a ReadPolicy,
                         key: &'a Key<'a>)
                         -> AerospikeResult<Arc<Record>> {
        let mut command = try!(ReadHeaderCommand::new(policy, self.cluster.clone(), key));
        try!(command.execute());
        Ok(command.record.as_ref().unwrap().clone())
    }

    pub fn put<'a, 'b>(&'a self,
                         policy: &'a WritePolicy,
                         key: &'a Key<'a>,
                         bins: &'a [&'b Bin])
                         -> AerospikeResult<()> {
        let mut command = try!(WriteCommand::new(policy, self.cluster.clone(), key, bins, operation::WRITE));
        command.execute()
    }

    pub fn add<'a, 'b>(&'a self,
                         policy: &'a WritePolicy,
                         key: &'a Key<'a>,
                         bins: &'a [&'b Bin])
                         -> AerospikeResult<()> {
        let mut command = try!(WriteCommand::new(policy, self.cluster.clone(), key, bins, operation::ADD));
        command.execute()
    }

    pub fn append<'a, 'b>(&'a self,
                         policy: &'a WritePolicy,
                         key: &'a Key<'a>,
                         bins: &'a [&'b Bin])
                         -> AerospikeResult<()> {
        let mut command = try!(WriteCommand::new(policy, self.cluster.clone(), key, bins, operation::APPEND));
        command.execute()
    }

    pub fn prepend<'a, 'b>(&'a self,
                         policy: &'a WritePolicy,
                         key: &'a Key<'a>,
                         bins: &'a [&'b Bin])
                         -> AerospikeResult<()> {
        let mut command = try!(WriteCommand::new(policy, self.cluster.clone(), key, bins, operation::PREPEND));
        command.execute()
    }

    pub fn delete<'a, 'b>(&'a self,
                         policy: &'a WritePolicy,
                         key: &'a Key<'a>)
                         -> AerospikeResult<bool> {
        let mut command = try!(DeleteCommand::new(policy, self.cluster.clone(), key));
        try!(command.execute());
        Ok(command.existed)
    }

    pub fn touch<'a, 'b>(&'a self,
                         policy: &'a WritePolicy,
                         key: &'a Key<'a>)
                         -> AerospikeResult<()> {
        let mut command = try!(TouchCommand::new(policy, self.cluster.clone(), key));
        command.execute()
    }

    pub fn exists<'a, 'b>(&'a self,
                         policy: &'a WritePolicy,
                         key: &'a Key<'a>)
                         -> AerospikeResult<bool> {
        let mut command = try!(ExistsCommand::new(policy, self.cluster.clone(), key));
        try!(command.execute());
        Ok(command.exists)
    }

    pub fn operate<'a, 'b>(&'a self,
                         policy: &'a WritePolicy,
                         key: &'a Key<'a>,
                         ops: &'a[operation::Operation<'a>])
                         -> AerospikeResult<Arc<Record>> {
        let mut command = try!(OperateCommand::new(policy, self.cluster.clone(), key, ops));
        try!(command.execute());
        Ok(command.read_command.record.as_ref().unwrap().clone())
    }


}
