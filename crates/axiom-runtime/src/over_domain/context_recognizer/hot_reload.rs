// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Горячая перезагрузка — обработка UCL команды RefreshPrimitiveScan.
// Источник: ContextRecognizer_V5_0.md §11

use axiom_ucl::{OpCode, UclCommand};

/// Построить UCL-команду RefreshPrimitiveScan.
pub fn make_refresh_primitive_scan(target_id: u32) -> UclCommand {
    UclCommand::new(OpCode::RefreshPrimitiveScan, target_id, 100, 0)
}

/// Построить UCL-команду QueryDepthDistribution для Workstation.
pub fn make_query_depth_distribution(target_id: u32, octant: u8) -> UclCommand {
    let payload = axiom_ucl::QueryDepthDistributionPayload { octant, reserved: [0; 47] };
    UclCommand::new(OpCode::QueryDepthDistribution, target_id, 80, 0).with_payload(&payload)
}

/// Построить UCL-команду ResetDepthForFrame (debug, через GUARDIAN).
pub fn make_reset_depth_for_frame(sutra_id: u32) -> UclCommand {
    let payload = axiom_ucl::ResetDepthForFramePayload { sutra_id, reserved: [0; 44] };
    UclCommand::new(OpCode::ResetDepthForFrame, 0, 200, 0).with_payload(&payload)
}
