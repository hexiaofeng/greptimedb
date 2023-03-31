// Copyright 2023 Greptime Team
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

use common_error::prelude::*;
use tokio::task::JoinError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("Failed to build runtime, source: {}", source))]
    BuildRuntime {
        source: std::io::Error,
        backtrace: Backtrace,
    },
    #[snafu(display("Repeated task {} not started yet", name))]
    IllegalState { name: String, backtrace: Backtrace },

    #[snafu(display(
        "Failed to wait for repeated task {} to stop, source: {}",
        name,
        source
    ))]
    WaitGcTaskStop {
        name: String,
        source: JoinError,
        backtrace: Backtrace,
    },
}

impl ErrorExt for Error {
    fn backtrace_opt(&self) -> Option<&Backtrace> {
        ErrorCompat::backtrace(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
