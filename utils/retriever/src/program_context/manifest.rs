// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::Dependency;
use serde::{Deserialize, Serialize};

// Struct representation of program's `program.json` specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    program: String,
    version: String,
    description: String,
    license: String,
    dependencies: Option<Vec<Dependency>>,
}

impl Manifest {
    pub fn program(&self) -> &String {
        &self.program
    }

    pub fn dependencies(&self) -> &Option<Vec<Dependency>> {
        &self.dependencies
    }
}
