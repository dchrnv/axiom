// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Debug sizes for cross-spec validation

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn debug_print_sizes() {
        println!("Token size: {} (expected: 64)", std::mem::size_of::<Token>());
        println!("Connection size: {} (expected: 64)", std::mem::size_of::<Connection>());
        println!("Event size: {} (expected: 32)", std::mem::size_of::<Event>());
        println!("DynamicTrace size: {} (expected: 32)", std::mem::size_of::<DynamicTrace>());
        println!("DomainConfig size: {} (expected: 128)", std::mem::size_of::<DomainConfig>());
        
        println!("Token align: {} (expected: 64)", std::mem::align_of::<Token>());
        println!("Connection align: {} (expected: 64)", std::mem::align_of::<Connection>());
        println!("Event align: {} (expected: 32)", std::mem::align_of::<Event>());
        println!("DynamicTrace align: {} (expected: 32)", std::mem::align_of::<DynamicTrace>());
        println!("DomainConfig align: {} (expected: 128)", std::mem::align_of::<DomainConfig>());
        
        // Token field offsets
        println!("Token sutra_id offset: {}", std::mem::offset_of!(Token, sutra_id));
        println!("Token domain_id offset: {}", std::mem::offset_of!(Token, domain_id));
        println!("Token type_flags offset: {}", std::mem::offset_of!(Token, type_flags));
        println!("Token position offset: {}", std::mem::offset_of!(Token, position));
        println!("Token velocity offset: {}", std::mem::offset_of!(Token, velocity));
        println!("Token target offset: {}", std::mem::offset_of!(Token, target));
        println!("Token valence offset: {}", std::mem::offset_of!(Token, valence));
        println!("Token mass offset: {}", std::mem::offset_of!(Token, mass));
        println!("Token temperature offset: {}", std::mem::offset_of!(Token, temperature));
        println!("Token state offset: {}", std::mem::offset_of!(Token, state));
        println!("Token lineage_hash offset: {}", std::mem::offset_of!(Token, lineage_hash));
        println!("Token momentum offset: {}", std::mem::offset_of!(Token, momentum));
        println!("Token resonance offset: {}", std::mem::offset_of!(Token, resonance));
        println!("Token last_event_id offset: {}", std::mem::offset_of!(Token, last_event_id));
        
        // Event field offsets
        println!("Event event_id offset: {}", std::mem::offset_of!(Event, event_id));
        println!("Event domain_id offset: {}", std::mem::offset_of!(Event, domain_id));
        println!("Event event_type offset: {}", std::mem::offset_of!(Event, event_type));
        println!("Event priority offset: {}", std::mem::offset_of!(Event, priority));
        println!("Event flags offset: {}", std::mem::offset_of!(Event, flags));
        println!("Event payload_hash offset: {}", std::mem::offset_of!(Event, payload_hash));
        println!("Event target_id offset: {}", std::mem::offset_of!(Event, target_id));
        println!("Event source_id offset: {}", std::mem::offset_of!(Event, source_id));
        println!("Event payload_size offset: {}", std::mem::offset_of!(Event, payload_size));
        println!("Event parent_event_id offset: {}", std::mem::offset_of!(Event, parent_event_id));
        
        // DynamicTrace field offsets
        println!("DynamicTrace x offset: {}", std::mem::offset_of!(DynamicTrace, x));
        println!("DynamicTrace y offset: {}", std::mem::offset_of!(DynamicTrace, y));
        println!("DynamicTrace z offset: {}", std::mem::offset_of!(DynamicTrace, z));
        println!("DynamicTrace weight offset: {}", std::mem::offset_of!(DynamicTrace, weight));
        println!("DynamicTrace frequency offset: {}", std::mem::offset_of!(DynamicTrace, frequency));
        println!("DynamicTrace created_at offset: {}", std::mem::offset_of!(DynamicTrace, created_at));
        println!("DynamicTrace last_update offset: {}", std::mem::offset_of!(DynamicTrace, last_update));
        println!("DynamicTrace source_type offset: {}", std::mem::offset_of!(DynamicTrace, source_type));
        println!("DynamicTrace source_id offset: {}", std::mem::offset_of!(DynamicTrace, source_id));
        println!("DynamicTrace flags offset: {}", std::mem::offset_of!(DynamicTrace, flags));
        println!("DynamicTrace resonance_class offset: {}", std::mem::offset_of!(DynamicTrace, resonance_class));
    }
}
