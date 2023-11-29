// Copyright (c) ONNX Project Contributors
//
// SPDX-License-Identifier: Apache-2.0
// 
// from onnx/common/constants.h

// For ONNX op/function registration.

// ONNX domains.
pub static AI_ONNX_ML_DOMAIN: &'static str = "ai.onnx.ml"; 
pub static AI_ONNX_TRAINING_DOMAIN: &'static str = "ai.onnx.training";
pub static AI_ONNX_PREVIEW_TRAINING_DOMAIN: &'static str = "ai.onnx.preview.training";

// The following two are equivalent in an onnx proto representation.
pub static ONNX_DOMAIN: &'static str = "";
pub static AI_ONNX_DOMAIN: &'static str = "ai.onnx";

pub fn normalize_domain(domain: &str) -> String {
    if domain == AI_ONNX_DOMAIN {
        ONNX_DOMAIN.to_string()
    }
    else {
        domain.to_string()
    }
}

pub fn is_onnx_domain(domain: &str) -> bool {
    domain == ONNX_DOMAIN || domain == AI_ONNX_DOMAIN
}

pub static OPTIONAL_VALUE: bool = false;

// For dimension denotation.
pub static DATA_BATCH: &'static str = "DATA_BATCH";
pub static DATA_CHANNEL: &'static str = "DATA_CHANNEL";
pub static DATA_TIME: &'static str = "DATA_TIME";
pub static DATA_FEATURE: &'static str = "DATA_FEATURE";
pub static FILTER_IN_CHANNEL: &'static str = "FILTER_IN_CHANNEL";
pub static FILTER_OUT_CHANNEL: &'static str = "FILTER_OUT_CHANNEL";
pub static FILTER_SPATIAL: &'static str = "FILTER_SPATIAL";

// For type denotation.
pub static TENSOR: &'static str = "TENSOR";
pub static IMAGE: &'static str = "IMAGE";
pub static AUDIO: &'static str = "AUDIO";
pub static TEXT: &'static str = "TEXT";
