// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// FractalChain — иерархия уровней AshtiCore.
//
// Этап 12A: Протокол 10→0
// maya_output(level N) → sutra_input(level N+1)

use axiom_core::{Token, Event};
use crate::AshtiCore;

/// Цепочка фрактальных уровней AshtiCore.
///
/// Уровни связаны протоколом 10→0:
/// `MAYA(level[n])` → `SUTRA(level[n+1])`
///
/// # Пример
///
/// ```rust
/// use axiom_domain::FractalChain;
///
/// let mut chain = FractalChain::new(2);
/// assert_eq!(chain.depth(), 2);
/// ```
pub struct FractalChain {
    levels: Vec<AshtiCore>,
}

impl FractalChain {
    /// Создать цепочку из `depth` уровней (level_id: 0, 1, ..., depth-1).
    pub fn new(depth: usize) -> Self {
        let levels = (0..depth)
            .map(|i| AshtiCore::new(i as u16))
            .collect();
        Self { levels }
    }

    /// Число уровней в цепочке.
    pub fn depth(&self) -> usize {
        self.levels.len()
    }

    /// Впрыснуть токен во входной SUTRA первого уровня.
    pub fn inject_input(&mut self, token: Token) -> Result<usize, crate::CapacityExceeded> {
        self.levels[0].set_sutra_input(token)
    }

    /// Забрать токен из MAYA последнего уровня.
    pub fn take_output(&mut self) -> Option<Token> {
        self.levels.last_mut()?.take_maya_output()
    }

    /// Один тик всей цепочки: тик каждого уровня + передача maya→sutra между уровнями.
    ///
    /// Порядок:
    /// 1. Тик уровня 0
    /// 2. MAYA(0) → SUTRA(1)
    /// 3. Тик уровня 1
    /// 4. MAYA(1) → SUTRA(2)
    /// ...
    ///
    /// Возвращает все физические события всех уровней.
    pub fn tick(&mut self) -> Vec<Event> {
        let mut all_events = Vec::new();

        for i in 0..self.levels.len() {
            // Тик текущего уровня
            let events = self.levels[i].tick();
            all_events.extend(events);

            // Передать выход MAYA уровня i во вход SUTRA уровня i+1
            if i + 1 < self.levels.len() {
                while let Some(token) = self.levels[i].take_maya_output() {
                    let _ = self.levels[i + 1].set_sutra_input(token);
                }
            }
        }

        all_events
    }

    /// Обменяться навыками между всеми уровнями цепочки.
    ///
    /// Каждый уровень импортирует навыки всех остальных уровней.
    /// Возвращает общее число импортированных навыков.
    pub fn exchange_skills(&mut self) -> usize {
        // Собрать все навыки со всех уровней
        let all_skills: Vec<_> = self.levels.iter()
            .flat_map(|lvl| lvl.export_skills())
            .collect();

        // Импортировать во все уровни (дубли отфильтровываются внутри import_batch)
        let mut total = 0;
        for lvl in self.levels.iter_mut() {
            total += lvl.import_skills(&all_skills);
        }
        total
    }

    /// Доступ к уровню по индексу.
    pub fn level(&self, index: usize) -> Option<&AshtiCore> {
        self.levels.get(index)
    }

    /// Mutable доступ к уровню по индексу.
    pub fn level_mut(&mut self, index: usize) -> Option<&mut AshtiCore> {
        self.levels.get_mut(index)
    }
}
