// Copyright (C) 2013-2020 Blockstack PBC, a public benefit corporation
// Copyright (C) 2020 Stacks Open Internet Foundation
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs;
use std::io;
use std::io::prelude::*;

use crate::chainstate::stacks::db::*;
use crate::chainstate::stacks::Error;
use crate::chainstate::stacks::*;

use std::path::{Path, PathBuf};

use crate::util_lib::db::Error as db_error;
use crate::util_lib::db::{query_count, query_rows, DBConn};

use crate::util_lib::strings::StacksString;

use stacks_common::util::hash::to_hex;

use crate::chainstate::burn::db::sortdb::*;

use crate::net::Error as net_error;

use clarity::vm::types::{PrincipalData, QualifiedContractIdentifier, StandardPrincipalData};

use clarity::vm::contexts::{AssetMap, OwnedEnvironment};

use clarity::vm::analysis::run_analysis;
use clarity::vm::ast::build_ast_with_rules;
use clarity::vm::types::{AssetIdentifier, Value};

pub use clarity::vm::analysis::errors::CheckErrors;
use clarity::vm::errors::Error as clarity_vm_error;

use clarity::vm::database::ClarityDatabase;

use clarity::vm::contracts::Contract;

use crate::clarity_vm::clarity::ClarityConnection;

impl StacksChainState {
    pub fn get_contract<T: ClarityConnection>(
        clarity_tx: &mut T,
        contract_id: &QualifiedContractIdentifier,
    ) -> Result<Option<Contract>, Error> {
        clarity_tx
            .with_clarity_db_readonly(|ref mut db| match db.get_contract(contract_id) {
                Ok(c) => Ok(Some(c)),
                Err(clarity_vm_error::Unchecked(CheckErrors::NoSuchContract(_))) => Ok(None),
                Err(e) => Err(clarity_error::Interpreter(e)),
            })
            .map_err(Error::ClarityError)
    }

    pub fn get_data_var<T: ClarityConnection>(
        clarity_tx: &mut T,
        contract_id: &QualifiedContractIdentifier,
        data_var: &str,
    ) -> Result<Option<Value>, Error> {
        let epoch = clarity_tx.get_epoch();
        clarity_tx
            .with_clarity_db_readonly(|ref mut db| {
                match db.lookup_variable_unknown_descriptor(contract_id, data_var, &epoch) {
                    Ok(c) => Ok(Some(c)),
                    Err(clarity_vm_error::Unchecked(CheckErrors::NoSuchDataVariable(_))) => {
                        Ok(None)
                    }
                    Err(e) => Err(clarity_error::Interpreter(e)),
                }
            })
            .map_err(Error::ClarityError)
    }
}
