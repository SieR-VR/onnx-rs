// Copyright (c) ONNX Project Contributors
//
// SPDX-License-Identifier: Apache-2.0
// 
// from onnx/common/status.h, onnx/common/status.cc

use std::boxed::Box;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum StatusCategory {
    None,
    Checker,
    Optimizer,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum StatusCode {
    Ok,
    Fail,
    InvalidArgument,
    InvalidProtobuf,
}

struct State {
    category: StatusCategory,
    code: StatusCode,
    msg: String,
}

impl State {
    pub fn new(category: StatusCategory, code: StatusCode, msg: String) -> State {
        State {
            category,
            code,
            msg,
        }
    }
}

pub struct Status {
    state_: Box<State>,
}

impl Status {
    pub fn new(category: StatusCategory, code: StatusCode, msg: Option<String>) -> Status {
        Status {
            state_: Box::new(State::new(category, code, msg.unwrap_or("".to_string()))),
        }
    }

    pub fn is_ok(&self) -> bool {
        self.state_.code == StatusCode::Ok
    }

    pub fn category(&self) -> StatusCategory {
        self.state_.category
    }

    pub fn code(&self) -> StatusCode {
        self.state_.code
    }

    pub fn error_message(&self) -> &String {
        &self.state_.msg
    }

    pub fn to_string(&self) -> String {
        if self.state_.code == StatusCode::Ok {
            return "OK".to_string();
        }

        return format!(
            "{} : {} : {} : {}",
            match self.state_.category {
                StatusCategory::None => "",
                StatusCategory::Checker => "[CheckerError]",
                StatusCategory::Optimizer => "[OptimizerError]",
            },
            match self.state_.code {
                StatusCode::Ok => 0,
                StatusCode::Fail => 1,
                StatusCode::InvalidArgument => 2,
                StatusCode::InvalidProtobuf => 3,
            },
            match self.state_.code {
                StatusCode::Ok => "GENERAL_ERROR",
                StatusCode::Fail => "FAIL",
                StatusCode::InvalidArgument => "INVALID_ARGUMENT",
                StatusCode::InvalidProtobuf => "INVALID_PROTOBUF",
            },
            self.state_.msg
        );
    }
}
