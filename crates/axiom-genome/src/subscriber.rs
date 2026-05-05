// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys

use crate::genome::Genome;
use crate::types::ResourceId;

/// Forward-compatibility trait для будущей эволюции GENOME.
///
/// В V1.0 GENOME неизменяем — `on_genome_update` не вызывается никогда.
/// Trait существует для обеспечения совместимости с будущими версиями.
pub trait GenomeSubscriber {
    /// Вызывается при обновлении GENOME (будущая функциональность, V1.0: не используется).
    fn on_genome_update(&mut self, new_genome: &Genome);

    /// Возвращает ресурсы, на которые подписан модуль.
    fn subscribed_resources(&self) -> &[ResourceId];
}
