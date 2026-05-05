// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys

use crate::genome::Genome;
use crate::types::{DataType, ModuleId, Permission, ResourceId, MAX_MODULES, MAX_RESOURCES};

/// Предвычисленные матрицы для O(1) lookup.
///
/// Строится один раз при инициализации из `Genome`.
/// Доступ: `access_matrix[module as usize][resource as usize]`
pub struct GenomeIndex {
    /// [ModuleId][ResourceId] -> Permission
    access_matrix: [[Permission; MAX_RESOURCES]; MAX_MODULES],
    /// [SourceModuleId][TargetModuleId] -> bool (есть ли любой протокол)
    protocol_matrix: [[bool; MAX_MODULES]; MAX_MODULES],
}

impl GenomeIndex {
    /// Построить индекс из Genome.
    pub fn build(genome: &Genome) -> Self {
        let mut access_matrix = [[Permission::None; MAX_RESOURCES]; MAX_MODULES];
        let mut protocol_matrix = [[false; MAX_MODULES]; MAX_MODULES];

        for rule in &genome.access_rules {
            let m = rule.module as usize;
            let r = rule.resource as usize;
            if m < MAX_MODULES && r < MAX_RESOURCES {
                access_matrix[m][r] = rule.permission;
            }
        }

        for rule in &genome.protocol_rules {
            let s = rule.source as usize;
            let t = rule.target as usize;
            if s < MAX_MODULES && t < MAX_MODULES {
                protocol_matrix[s][t] = true;
            }
        }

        Self {
            access_matrix,
            protocol_matrix,
        }
    }

    /// O(1) проверка права доступа.
    pub fn check_access(
        &self,
        module: ModuleId,
        resource: ResourceId,
        required: Permission,
    ) -> bool {
        self.access_matrix[module as usize][resource as usize] >= required
    }

    /// O(1) проверка наличия протокола между двумя модулями.
    pub fn check_protocol(&self, source: ModuleId, target: ModuleId) -> bool {
        self.protocol_matrix[source as usize][target as usize]
    }

    /// O(1) проверка конкретного маршрута по типу данных.
    pub fn check_protocol_typed(
        &self,
        genome: &Genome,
        source: ModuleId,
        target: ModuleId,
        data_type: DataType,
    ) -> bool {
        genome
            .protocol_rules
            .iter()
            .any(|r| r.source == source && r.target == target && r.data_type == data_type)
    }
}
