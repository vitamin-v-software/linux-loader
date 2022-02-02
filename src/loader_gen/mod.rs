// Copyright (c) 2019 Intel Corporation. All rights reserved.
// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
//
// Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE-BSD-3-Clause file.
//
// SPDX-License-Identifier: Apache-2.0 AND BSD-3-Clause

//! Bindgen autogenerated structs for boot parameters.

#![cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// Keep this until https://github.com/rust-lang/rust-bindgen/issues/1651 is fixed.
#![cfg_attr(test, allow(deref_nullptr, unaligned_references))]

mod x86_64;
pub use x86_64::*;
