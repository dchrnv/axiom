| axiom-neural | 28 |# AXIOM Status
| axiom-neural | 28 |
| axiom-neural | 28 |**Обновлено:** 2026-06-12
| axiom-neural | 28 |**Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)
| axiom-neural | 28 |
| axiom-neural | 28 |---
| axiom-neural | 28 |
| axiom-neural | 28 |## Текущее состояние
| axiom-neural | 28 |
| axiom-neural | 28 |**1536 тестов (all features), TEST-TD-01 — pre-existing (DEFERRED)**
| axiom-neural | 28 |
| axiom-neural | 28 |Neural Integration Этап 1 ✅ (2026-06-12): axiom-neural крейт + ReactivationDepthModel (1D-CNN, 13K params,
| axiom-neural | 28 |  pure Rust: rustfft+ndarray) + training_data.jsonl в OBS (каждые 200 тиков) + NeuralReactivationDepthAdvisor
| axiom-neural | 28 |  (mode=Rule/Neural, 1ms timeout, fallback) + Neural Depth Advisor панель в Workstation.
| axiom-neural | 28 |  Режим по умолчанию: Rule. Neural mode ждёт обучения (NEURAL-TD-01/02 в DEFERRED).
| axiom-neural | 28 |
| axiom-neural | 28 |OBS-ACC-02 ✅ (2026-06-11): регрессия anchor-детекции logic_deductive/morality_duty → 100%.
| axiom-neural | 28 |  Причина: OBS-ACC-01 переименовал теги meta→abstractions, создав ничьи через общие алиасы.
| axiom-neural | 28 |  abstractions/primitives.yaml: удалены aliases "форма" (abstraction_pattern), "теорема/следствие/
| axiom-neural | 28 |  доказательство" (abstraction_schema), "закон/принцип" (abstraction_theory).
| axiom-neural | 28 |  logic/primitives.yaml: удалён alias "закон" из logic_rule (слишком амбивалентен).
| axiom-neural | 28 |  anchor_match.rs: +2 регрессионных теста (logic_deductive/morality_duty).
| axiom-neural | 28 |  membrane_blend_factor: 0.5→0.7 (per спека §9: entropy есть но мал).
| axiom-neural | 28 |  genome.rs + genome.yaml синхронизированы. OBS: 19/19 × 100% accuracy, per-text=100%.
| axiom-neural | 28 |
| axiom-neural | 28 |refactor(axial-evaluator) ✅ (2026-06-11): убрать mean_y workaround — мембраны дают честный valence.
| axiom-neural | 28 |  axial_evaluator/mod.rs: OBS-AX-01 workaround (mean_y при density=0) удалён.
| axiom-neural | 28 |  Domain_Membrane_Profiles_V1_0 теперь даёт реальную валентность токенам (±40), pos_val/neg_val работают честно.
| axiom-neural | 28 |
| axiom-neural | 28 |Domain_Membrane_Profiles_V1_0 ✅ (2026-06-11): мембранная трансформация токенов по природе домена.
| axiom-neural | 28 |  axiom-genome: MembraneProfile {mass_in,valence_in,temp_in,blend_factor?}; Genome += membrane_profiles
| axiom-neural | 28 |  (8 доменов 101–108) + membrane_blend_factor=0.5. genome.yaml: секция membrane_profiles.
| axiom-neural | 28 |  axiom-arbiter: axiom-genome как зависимость; Arbiter += membrane_profiles+blend_factor;
| axiom-neural | 28 |  configure_membranes(); membrane_transform() — чистая функция: blend_u8/i8 + clamp (mass≥1, temp≥1).
| axiom-neural | 28 |  route_to_ashti: перед process_token → membrane_transform (slow path only, fast path без изменений).
| axiom-neural | 28 |  axiom-domain: AshtiCore::apply_membrane_profiles() → arbiter.configure_membranes().
| axiom-neural | 28 |  axiom-runtime: AxiomEngine::try_new вызывает apply_membrane_profiles из genome.
| axiom-neural | 28 |  config/presets/domains/: logic.yaml membrane_state 2→3 (MEMBRANE_ADAPTIVE, фикс); 
| axiom-neural | 28 |  void.yaml quantum_noise→150; shadow.yaml resonance_freq→400; logic.yaml resonance_freq→200.
| axiom-neural | 28 |  9 unit-тестов: transform_execution, transform_dream, symmetry, clamping×3, factor_zero, 
| axiom-neural | 28 |  preserves_non_physical_fields, per_domain_factor_override.
| axiom-neural | 28 |  test_from_yaml_matches_default расширен: проверяет 8 профилей + blend_factor.
| axiom-neural | 28 |
| axiom-neural | 28 |OBS-AX-01 ✅ (2026-06-07): AxialEvaluator Y-ось — Eros/Thanatos из позиции участников.
| axiom-neural | 28 |  axial_evaluator/mod.rs: при density=0 и valence=0 Y-ось вычисляется из mean_y позиции
| axiom-neural | 28 |  участников (по спеке Domain V1.3: Y+ = Eros, Y- = Thanatos).
| axiom-neural | 28 |  Исправляет: thanatos = 255 - density = 255 - 0 = 255 → Y всегда Thanatos (O2/O5/O6 = 0).
| axiom-neural | 28 |  Теперь: Y > 3860 → Eros → активируются O1/O5 аналитически для high-Y контента.
| axiom-neural | 28 |  2 новых теста: y_axis_eros_for_high_y / y_axis_thanatos_for_low_y.
| axiom-neural | 28 |  Dionysus (X-) требует высокой entropy (diverse позиций) — отложено.
| axiom-neural | 28 |
| axiom-neural | 28 |TENS-TD-01 ✅ (2026-06-07): TensionTrace после разрешения дилеммы.
| axiom-neural | 28 |  engine.rs t%7: drain_resolution_tensions() → add_tension_trace(temp=127, 1270 тиков).
| axiom-neural | 28 |  context_recognizer/mod.rs: pending_resolution_tensions: Vec<u64> → drain_resolution_tensions().
| axiom-neural | 28 |  После drain_pending_crystallizations() для каждой resolved дилеммы → push(tick).
| axiom-neural | 28 |  arbiter/src/lib.rs: TENSION_DECAY 10→1 (нормальные трейсы ~630 тиков, resolution ~1270).
| axiom-neural | 28 |  cognitive_depth_13b_tests.rs: test_pulse_cools_traces адаптирован под DECAY=1.
| axiom-neural | 28 |  3 новых теста: resolution_tension_emitted, drain_empty_initially, drain_clears_buffer.
| axiom-neural | 28 |  2 integration теста: resolution_tension_created_in_experience + tension_decay_persist_longer.
| axiom-neural | 28 |
| axiom-neural | 28 |OBS-ACC-01 ✅ (2026-06-07): Точность обнаружения подсистем — abstractions/morality/writing.
| axiom-neural | 28 |  anchor.rs: "abstractions" добавлен в SUBSYSTEM_NAMES (Path 1 теперь видит abstraction_ якоря).
| axiom-neural | 28 |  decomposition_table.rs: subsystem_from_anchor_id обрабатывает "abstraction_" префикс (Path 2).
| axiom-neural | 28 |  time/primitives.yaml: time_before word "до"→"прежде" (удалён ложный матч предлога).
| axiom-neural | 28 |  logic/primitives.yaml: logic_negation alias "не" удалён (слишком частое слово, ложные позитивы).
| axiom-neural | 28 |  mathematics/primitives.yaml: math_relation alias "теорема" удалён (не тип отношения).
| axiom-neural | 28 |  values/primitives.yaml: val_beneficial word "благо"→"польза" (prevents tie с morality в утилитарных текстах).
| axiom-neural | 28 |  abstractions/primitives.yaml: теги "meta"→"abstractions"; abstraction_theory += бесконечность/теорема;
| axiom-neural | 28 |    abstraction_category += множество; abstraction_schema += теорема/лемма.
| axiom-neural | 28 |  morality/primitives.yaml: новый якорь moral_utilitarian (word "утилитаризм").
| axiom-neural | 28 |  writing/primitives.yaml: новый якорь prim_style (word "краткость").
| axiom-neural | 28 |  anchor_match.rs: 2 новых теста (subsystem_from_anchor_id + 5 интеграционных кейсов).
| axiom-neural | 28 |  OBS corpus_showcase: abstractions/morality/writing_metaphor/writing_style → ожидается ✓ 100%.
| axiom-neural | 28 |
| axiom-neural | 28 |DIL-TD-01 ✅ (2026-06-07): Dilemma Resolution Pipeline — дилеммы наконец разрешаются.
| axiom-neural | 28 |  context_recognizer/mod.rs: intensity decay (×0.997/CR-тик) + resolution conditions в on_tick():
| axiom-neural | 28 |  Type III (ValueConflict): dominant_persistence > 0.75 AND intensity < 0.15 → ContextualPriority.
| axiom-neural | 28 |  Type IV (OntologicalConflict): age ≥ 500 тиков AND entropy_gradient ≈ 0 → Complementarity.
| axiom-neural | 28 |  Fallback: intensity < 0.02 → принудительное разрешение. Кристаллизация в EXPERIENCE через
| axiom-neural | 28 |  drain_pending_crystallizations() → crystallize_to_experience_commands() в каждом on_tick().
| axiom-neural | 28 |  OBS corpus_showcase: resolved=64 (MAX_RESOLVED), active=0. 3 новых теста в dilemma_store.rs.
| axiom-neural | 28 |  Также: compute_confidence tolerance ±20→±8 (maya_processor.rs) + min_coherence 153→200
| axiom-neural | 28 |  (maya.yaml): avg coherence 1.000→0.750, multi-pass events появились (было 0/∞ → 16/45K).
| axiom-neural | 28 |
| axiom-neural | 28 |SEN-TD-01 Фаза F ✅ (2026-06-05): BroadcastSnapshot удалён — SensoriumState единственный источник.
| axiom-neural | 28 |  axiom-broadcasting/snapshot.rs: build_system_snapshot прямые запросы к &AxiomEngine (без bs).
| axiom-neural | 28 |  engine.rs: snapshot_for_broadcast() и domain_summaries() удалены; last_dream_summary всегда pub.
| axiom-neural | 28 |  broadcast.rs: BroadcastSnapshot, DomainSummary, DreamPhaseSnapshot, ActiveCycleSnapshot удалены;
| axiom-neural | 28 |  остались LastDreamSummary, DomainDetailSnapshot, TokenSnapshot, ConnectionSnapshot.
| axiom-neural | 28 |  axiom-agent: BroadcastSnapshot → Option<SensoriumState> в protocol/tick_loop/ws/rest.
| axiom-neural | 28 |  Тесты: broadcast_tests.rs и dream_cli_tests.rs переписаны под SensoriumState.
| axiom-neural | 28 |
| axiom-neural | 28 |SEN-TD-01 Фаза B ✅ (2026-06-05): SensoriumState публикуется через BroadcastHandle.
| axiom-neural | 28 |  SensoriumState + все типы: добавлен Serialize. axiom-broadcasting: serde_json dependency;
| axiom-neural | 28 |  BroadcastHandle.sensorium_live (pre-serialized JSON) + update_sensorium() + latest_sensorium_json().
| axiom-neural | 28 |  axiom-node tick.rs: update_sensorium() после каждого snapshot. http.rs WS bridge:
| axiom-neural | 28 |  при connect отправляет {"type":"Sensorium","data":{...}} вместе с SystemSnapshot.
| axiom-neural | 28 |
| axiom-neural | 28 |SEN-TD-01 Фаза A ✅ (2026-06-05): SensoriumState поглощает поля BroadcastSnapshot.
| axiom-neural | 28 |  `state.rs`: SensoriumDomainSummary, SensoriumDreamSummary; новые поля SensoriumState:
| axiom-neural | 28 |  trace_count, tension_count, domain_summaries, last_crystallization_tick,
| axiom-neural | 28 |  guardian_vetoes_since_wake, cross_modal_candidates, last_dream_summary.
| axiom-neural | 28 |  SensoriumView расширен; collect_pulse заполняет все поля каждый тик.
| axiom-neural | 28 |  engine.rs: pre-compute domain_summaries + scalar fields до построения SensoriumView.
| axiom-neural | 28 |
| axiom-neural | 28 |Shell-TD-02 ✅ (2026-06-04): shell bonus в resonance_search.
| axiom-neural | 28 |  `crates/axiom-arbiter/src/experience.rs`: shell_registry: HashMap<u32,[u8;8]> в Experience;
| axiom-neural | 28 |  set_shell_registry() — заполняется из engine.inject_anchor_tokens при boot.
| axiom-neural | 28 |  pattern_similarity() расширена: shell_cosine([u8;8],[u8;8]) → 15% бонус/штраф к score.
| axiom-neural | 28 |  Идентичные профили → ×1.0 (max); ортогональные → ×0.85 (min); нет в registry → ×0.925 (нейтраль).
| axiom-neural | 28 |  6 unit-тестов: cosine/neutral/bonus/penalty/neutral-registry/propagation.
| axiom-neural | 28 |  Вызов: self.ashti.experience_mut().set_shell_registry() в inject_anchor_tokens.
| axiom-neural | 28 |
| axiom-neural | 28 |PRIM-TD-03 ✅ (2026-06-04): Subsystem Gravity — семантическое притяжение/отталкивание.
| axiom-neural | 28 |  `crates/axiom-runtime/src/subsystem_gravity.rs` (новый): SubsystemGravityRule, apply_subsystem_gravity,
| axiom-neural | 28 |  build_rules_from_anchor_set. Формула: нормализованный вектор × BASE_FORCE(16) × strength_factor.
| axiom-neural | 28 |  Values: val_beneficial pull(0.20) / val_harmful push(0.20), без радиуса (весь MAYA домен).
| axiom-neural | 28 |  Abstractions: abstraction_theory/constructor pull(0.08), radius=8000 (локальный подъём).
| axiom-neural | 28 |  Вызов в engine cold path: subsystem_gravity_interval=500 (TickSchedule). НЕ в apply_gravity_batch.
| axiom-neural | 28 |  7 unit-тестов: pull/push/radius/at-anchor/no-collapse/locked-skip/empty-set.
| axiom-neural | 28 |  AxiomEngine.subsystem_gravity_rules: строится в inject_anchor_tokens, immutable в runtime.
| axiom-neural | 28 |
| axiom-neural | 28 |PRIM-TD-05 ✅ (2026-06-03): L0 уровень для abstraction_raw.
| axiom-neural | 28 |  `config/anchors/abstractions/primitives.yaml`: `layer: L0` добавлен в `abstraction_raw` (C0).
| axiom-neural | 28 |  `anchor.rs` all_anchors(): L0-якоря из subsystems теперь исключены из match_text() — как и L0
| axiom-neural | 28 |  из perceptual. abstraction_raw (сырой сигнал) больше не матчится в тексте: правильно, C0 ≠ языковой концепт.
| axiom-neural | 28 |
| axiom-neural | 28 |TemporalPerceptor ✅ (PRIM-TD-04, 2026-06-03): темпоральные маркеры в тексте → time_*-якоря → SUTRA.
| axiom-neural | 28 |  `crates/axiom-agent/src/perceptors/temporal.rs`: temporal_anchor_stable_id (FNV-1a, бит 28,
| axiom-neural | 28 |  диапазон 0x1000_0001..0x1FFF_FFFF); 7 паттернов (time_before/after/simultaneous/duration/
| axiom-neural | 28 |  periodic/irreversible/horizon); word + aliases (case-insensitive); stable_id в reserved[0..4].
| axiom-neural | 28 |  Интеграция: TemporalPerceptor::new(anchor_set.get_subsystem("time")); 10 unit-тестов.
| axiom-neural | 28 |
| axiom-neural | 28 |Cross-Modal Binding V1.0 — pipeline замкнут (2026-06-03):
| axiom-neural | 28 |  **vision_anchor_stable_id** (бит 29, FNV-1a, диапазон 0x2000_0001..0x3FFF_FFFF): L0VisionPerceptor
| axiom-neural | 28 |  теперь записывает стабильный sutra_id в reserved[0..4] payload (как TextPerceptor text_stable_id).
| axiom-neural | 28 |  Фикс: один и тот же визуальный якорь всегда получает один sutra_id → FrameWeaver видит стабильные
| axiom-neural | 28 |  Vision токены → Vision Frames кристаллизуются → CrossModalDetector находит Text+Vision пары.
| axiom-neural | 28 |  **CMB-TD-03**: CrossModalBondProposed в EngineEvent (axiom-protocol); pending_cross_modal_bond_events
| axiom-neural | 28 |  в AxiomEngine; drain_cross_modal_bond_events(); tick.rs публикует WS-событие при создании bond;
| axiom-neural | 28 |  BroadcastSnapshot += cross_modal_candidates + cross_modal_bonds.
| axiom-neural | 28 |  **3 integration теста** CR: накопление candidates, bond after threshold, same-modality no candidate.
| axiom-neural | 28 |
| axiom-neural | 28 |Waves V1.0 ✅ (2026-06-03): внутренний ветер. ModuleId::Waves=22, MAX_MODULES=23.
| axiom-neural | 28 |  `over_domain/waves/`: Impulse (source/target/pull_strength/age/raise_count), ImpulseSource A/B/C,
| axiom-neural | 28 |  internal_dominance_factor (0..1, плавный, вход перебивает), WAVES_TICK_INTERVAL=19.
| axiom-neural | 28 |  Источник A: активные дилеммы (intensity × age-бонус). Источник B: SutraDepth глубокий резонанс
| axiom-neural | 28 |  (max_depth > 500, не примитивы). Источник C: FrameWeaver candidates близкие к кристаллизации.
| axiom-neural | 28 |  Защиты: затухание повтора (DECAY_RATE=15), MAX_ACTIVE_IMPULSES=4, fatigue-потолок,
| axiom-neural | 28 |  DREAM сбрасывает (dream_reset: 75% силы). UCL: ReinforceFrame для Source B/C (Source A — skip).
| axiom-neural | 28 |  Тикает до Sensorium в wake-цикле. Sensorium видит impulses через WavesView.
| axiom-neural | 28 |
| axiom-neural | 28 |Sensorium V1.0 ✅ (2026-06-03): полный внутренний срез системы. ModuleId::Sensorium=21.
| axiom-neural | 28 |  `over_domain/sensorium/`: SensoriumState (4 группы полей §2), CollectionLevel (0-3),
| axiom-neural | 28 |  SensoriumSchedule (big_cycle=32), ConsumerRegistry, SensoriumExpression (детерминированная функция §11).
| axiom-neural | 28 |  collect() тикает последним в handle_tick_wake(), принимает SensoriumView (&-ссылки на все OD-компоненты).
| axiom-neural | 28 |  on_dream_wake() → schedule_memory_collection(). GENOME: Read на все ресурсы (инвариант навсегда).
| axiom-neural | 28 |  Параллельно TickSnapshot, не заменяет его (см. DEFERRED SEN-TD-01 → V2.0).
| axiom-neural | 28 |
| axiom-neural | 28 |Primitive_Nature_and_Connections_V1_0 ✅ (2026-05-30): spatial/causal L0 переведены из якорей-призраков
| axiom-neural | 28 |в Connection link_type определения (config/schema/link_types/); 0x09 Spatial добавлен в
| axiom-neural | 28 |semantic_contributions.yaml; perceptual_anchors() = 8 (только visual); primitives_nature.yaml создан.
| axiom-neural | 28 |
| axiom-neural | 28 |```
| axiom-neural | 28 |AxiomEngine
| axiom-neural | 28 |  ├── Genome (конституция, from_yaml, GenomeIndex O(1))
| axiom-neural | 28 |  ├── AshtiCore — 11 доменов (SUTRA=level*100 .. MAYA=level*100+10)
| axiom-neural | 28 |  │     ├── Arbiter (dual-path routing + Experience + Reflector + SkillSet + Internal Drive)
| axiom-neural | 28 |  │     ├── 11 × Domain (физика поля + CausalFrontier V2.0)
| axiom-neural | 28 |  │     └── 11 × DomainState (токены + связи)
| axiom-neural | 28 |  ├── Guardian (CODEX + GENOME: enforce_access, validate_reflex, ML filters)
| axiom-neural | 28 |  └── Over-Domain Layer:
| axiom-neural | 28 |        ├── OverDomainComponent trait (object-safe, on_tick → Result<Vec<UclCommand>, OverDomainError>)
| axiom-neural | 28 |        ├── Weaver trait (type Pattern, scan, propose_to_dream, check_promotion(tick))
| axiom-neural | 28 |        ├── FrameWeaver V1.3 ✅ — scan MAYA (0x08 Syntactic) → кристаллизация EXPERIENCE (109);
| axiom-neural | 28 |        │     FrameCandidate.shell_similarity: f32 — средн. косинусное сходство shell участников;
| axiom-neural | 28 |        │     avg_candidate_shell_similarity() → f32 — диагностика для OBS-снимков
| axiom-neural | 28 |        ├── AxialEvaluator V3.0 ✅ (tick=5, ModuleId=17) — Frame по осям X/Y/Z; 8 уровней; Corpus Callosum;
| axiom-neural | 28 |        │     V2: OctantStabilityTracker (ring 10, threshold 70%, min 5), ConflictPersistenceTracker (streak≥5);
| axiom-neural | 28 |        │     subsystem-aware level selection (subsystem_to_level); drain_pending_advisories() → Vec<Advisory>;
| axiom-neural | 28 |        │     AXIAL_EVALUATOR_SOURCE_ID=1; TrustConfig: OctantCorrection(0.70)/ConflictDiagnosis(0.60);
| axiom-neural | 28 |        │     V3: NarrativeOctantTracker (применяет advisory override); adaptive stability threshold;
| axiom-neural | 28 |        │     AxialStore::override_octant(sutra_id, octant) — advisory-override flag, AE уважает на следующем тике
| axiom-neural | 28 |        ├── ContextRecognizer V6.0 ✅ (tick=7, ModuleId=18) — SubsystemEnergy, InterpretationProfile, SutraDepthStore;
| axiom-neural | 28 |        │     V6 A: ActivityTrace (short=16/mid=64/long=256 ring-буферы), ActivityDynamics (entropy_gradient,
| axiom-neural | 28 |        │           oscillation_score, cascade_score, dominant_persistence), ActivitySignature classifier,
| axiom-neural | 28 |        │           ActivityAnalyzer (переименован из TransitionDetector);
| axiom-neural | 28 |        │     V6 B: SubsystemFatigue { activation_load, recovery_debt }, FatigueStore (V7-B2 → axiom-experience);
| axiom-neural | 28 |        │           effective_weight = base*(1-0.5*min(1,load/MAX)); DREAM: activation_load *= 0.35;
| axiom-neural | 28 |        │     TransitionMatrix ✅ (V7-B1) — [[f32; 16]; 16] матрица переходов; record(from, to) при смене
| axiom-neural | 28 |        │           доминанты; decay(0.995) на каждом тике; probability_of(from, to), most_likely_next(from);
| axiom-neural | 28 |        │           Unknown игнорируется; 7 unit-тестов; 3 CR-интеграционных теста
| axiom-neural | 28 |        │     directed_cascade_score ✅ (V7-C1) — ActivityDynamics.directed_cascade_score: f32;
| axiom-neural | 28 |        │           ActivityTrace::directed_cascade_score(matrix, threshold=0.20) → цепочка A→B→C
| axiom-neural | 28 |        │           где prob(A→B)≥T; classify() предпочитает если >0 (fallback на cascade_score); 5 тестов
| axiom-neural | 28 |        │     CompositeSubsystemProfile ✅ (V7-C2) — полный профиль с BidirectionalCoupling;
| axiom-neural | 28 |        │           detect_composite_profiles(recent, sigs, matrix, bi_threshold=0.15);
| axiom-neural | 28 |        │           composite_profiles() accessor в CR; V6 composite_suspects сохранён; 6 тестов
| axiom-neural | 28 |        │     SubsystemVersionStore ✅ (V7-D1) — version в FlatAnchorFile + AnchorSet.subsystem_versions;
| axiom-neural | 28 |        │           init()/check_migration()/drain_stale(); from_anchor_set инициализирует; 8 тестов
| axiom-neural | 28 |        │     SplitMergeDetector ✅ (V7-D2) — Split(load≥0.6·MAX + entropy≥1.5) / Merge(bidirectional≥0.25);
| axiom-neural | 28 |        │           split_merge_candidates() accessor; 9 unit-тестов; on_tick после fatigue.update()
| axiom-neural | 28 |        │     compute_raw_energies(&AshtiCore) → HashMap<SubsystemId, u8> — снимок энергий для OBS
| axiom-neural | 28 |        │     ActivityDynamics fix ✅ (2026-05-30) — CR on_tick: N=1 most-recent MAYA token (by last_event_id)
| axiom-neural | 28 |        │           вместо cumulative compute_energies; dominant_subsystem_confident (threshold 5e-9);
| axiom-neural | 28 |        │           AshtiCore::sleep_oldest_active_token(domain_id) — eviction при переполнении MAYA;
| axiom-neural | 28 |        │           E1-fix: valence=1 + retry on CapacityExceeded → динамика жива весь прогон
| axiom-neural | 28 |        │           (corpus_mixed 60K тиков: Cascade=1.00, Fill=16, Fatigue=4, Signature=Cascading)
| axiom-neural | 28 |        │     Morality detection ✅ (2026-05-30) — SUBSYSTEM_NAMES += "morality"; moral_ prefix в
| axiom-neural | 28 |        │           subsystem_from_anchor_id; word_signals для 7 moral_* якорей (moral_care/harm/fair/
| axiom-neural | 28 |        │           betrayal/loyalty/purity/desecration); SubsystemId::Morality в build_subsystem_refs;
| axiom-neural | 28 |        │           corpus_mixed: config/obs/corpus_mixed.yaml (диагностический корпус 15 текстов,
| axiom-neural | 28 |        │           типы A/Б/В, inject_every=20, stagger=5 тиков/шард)
| axiom-neural | 28 |        │     FrameCompositionStore ✅ (V7-A1) — иерархия Frame-композиций; detect_composed_of() — участники
| axiom-neural | 28 |        │       совпадающие с Frame-анкерами EXPERIENCE = родители; COMPOSITION_BOND (0x0901) в UCL;
| axiom-neural | 28 |        │       composition_level(anchor_id) → FrameComposition (C1Atom..C5Plus);
| axiom-neural | 28 |        │       FrameCandidate.composed_of: Vec<u32> — заполняется перед кристаллизацией; 10 новых тестов
| axiom-neural | 28 |        │     DilemmaStore V1.1 ✅ — хранит дилеммы типов III/IV/V (не I/II); max 8 active, ring-64 resolved;
| axiom-neural | 28 |        │       pending_crystallizations → drain → crystallize_to_experience_commands() → UCL (InjectToken+BondTokens);
| axiom-neural | 28 |        │       кристаллизация в EXPERIENCE domain (level*100+9); lineage_hash FNV-1a; resolution_valence;
| axiom-neural | 28 |        │       DilemmaType: DataConflict/ResourceTradeoff/ValueConflict/OntologicalConflict/Axiogenic
| axiom-neural | 28 |        ├── NeuralAdvisor V3.0 ✅ (tick=11, ModuleId=19) — все 5 слотов заполнены;
| axiom-neural | 28 |        │     depth: ReactivationDepthAdvisor; octant: DepthHistoryBiasAdvisor (DHB_MIN_DEPTH=800,
| axiom-neural | 28 |        │     DHB_MIN_ADVANTAGE=300); conflict: RuleBasedCorpusCallosumResolver (V2) / PatternLearningResolver (V3);
| axiom-neural | 28 |        │     subsystem: AnchorVotingAdvisor (AV_MIN_ENERGY=20, dominance≥0.50, dual-gap<0.15);
| axiom-neural | 28 |        │     emergent: DepthThresholdEmergentDetector; AdvisoryHistory (ring 32 per sutra_id);
| axiom-neural | 28 |        │     OctantAdvisorInput расширен: depth_per_octant[8] + reactivation_count;
| axiom-neural | 28 |        │     implements AdvisorySource → poll_advisories() → Vec<Advisory> с octant_hint;
| axiom-neural | 28 |        │     G1: DivergenceLog (ring 256) — расхождения advisory_octant ↔ analytic_octant (Hamming ≥ 2);
| axiom-neural | 28 |        │     G2: PatternLearningResolver — conflict slot, учится на AdvisoryHistory per-Frame;
| axiom-neural | 28 |        │     G3: NeuralAdvisorConfig — genome.yaml секция neural_advisor → per-advisor enable/disable
| axiom-neural | 28 |        └── OverDomainArbiter V3.0 ✅ (tick=13, ModuleId=20) — координатор advisory-источников;
| axiom-neural | 28 |              TrustConfig (Ignore/AutoApply/RequireConfirmation × min_confidence);
| axiom-neural | 28 |              V2: TrustConfig загружается из genome.yaml секции [arbiter.trust]; TTL ~1000 event_id
| axiom-neural | 28 |              (expires_at_event = created_at_event + 1000 → ArbiterOutcome::Expired + on_feedback);
| axiom-neural | 28 |              CognitiveProfile загружается из yaml (профили: balanced/analytic); AutoApply DepthHint при
| axiom-neural | 28 |              Control в геноме; PendingQueue → Workstation V2 (confirm/reject через HTTP + WS);
| axiom-neural | 28 |              ArbiterLog (ring buffer 500); on_boot читает ExperienceMemory/Control из генома;
| axiom-neural | 28 |              CognitiveProfile { octant_weights[8], init 1.0 }: scale_confidence(octant_idx, raw),
| axiom-neural | 28 |              update(idx, accepted) online learning rate=0.05; Advisory.octant_hint: Option<usize>
| axiom-neural | 28 |              scan_state (confidence из avg connection.strength), build_crystallization_commands,
| axiom-neural | 28 |              ReinforceFrame (lineage_hash dedup), build_promotion_commands (→ SUTRA STATE_LOCKED),
| axiom-neural | 28 |              CycleStrategy::Allow (default); restore_frame_from_anchor; UnfoldFrame handler;
| axiom-neural | 28 |              встроен в AxiomEngine (on_tick + drain_commands); FrameWeaverStats: unfold_requests;
| axiom-neural | 28 |              GENOME: on_boot enforcement (check_access для MAYA/Read, EXPERIENCE/ReadWrite, SUTRA/Control);
| axiom-neural | 28 |              RuleTrigger: StabilityReached, HighConfidence(f32), DreamCycle, RepeatedAssembly{window_ticks};
| axiom-neural | 28 |              min_participant_anchors cross-domain check; check_promotion(tick) — корректный min_age_ticks;
| axiom-neural | 28 |              V1.2: промоция → dream_propose(); V1.3: все RuleTrigger реализованы, GENOME enforcement;
| axiom-neural | 28 |              AxiomEngine: confirm_pending_advisory(advisory_id: u64), reject_pending_advisory(advisory_id: u64);
| axiom-neural | 28 |              V3: drain_octant_overrides() → pending octant overrides для AxialEvaluatorStorage;
| axiom-neural | 28 |              V3: feedback-буфер для незарегистрированных источников (AxialEvaluator source_id)
| axiom-neural | 28 |
| axiom-neural | 28 |DREAM Phase V1.1 ✅ — когнитивный сон: 4 состояния (Wake/FallingAsleep/Dreaming/Waking)
| axiom-neural | 28 |  ├── DreamScheduler — 3 триггера: Idle (порог idle тиков), Fatigue (0-255, 4 фактора), ExplicitCommand
| axiom-neural | 28 |  ├── FatigueTracker — composite score = Σ(factor × weight) / Σ(weight); отслеживает 4 показателя
| axiom-neural | 28 |  ├── DreamCycle — 3 этапа: Stabilization → Processing → Consolidation; DreamProposal (Promotion/HeavyCrystallization)
| axiom-neural | 28 |  ├── GUARDIAN: check_frame_anchor_sutra_write() — FRAME_ANCHOR в SUTRA только в DREAMING
| axiom-neural | 28 |  ├── GatewayPriority: Normal (игнорируется в DREAMING) / Critical (пробуждение) / Emergency (V2.0=Critical)
| axiom-neural | 28 |  ├── Gateway::with_config() — старт с загрузкой DreamConfig из axiom.yaml
| axiom-neural | 28 |  ├── CLI: :dream-stats / :force-sleep / :wake-up
| axiom-neural | 28 |  ├── BroadcastSnapshot расширен: dream_phase, dream_stats (FatigueStats, SchedulerStats, CycleStats)
| axiom-neural | 28 |  └── H1/H2: SubsystemCandidate discovery — cluster_emergent_primitives() → SubsystemCandidateStore;
| axiom-neural | 28 |        SubsystemLifecycleState: Proposed→Candidate→InReview→Active→Mature→Deprecated→Archived;
| axiom-neural | 28 |        ApproveSubsystemCandidate (UCL 5301): approve_with_rules(id, genome.emergent_subsystems);
| axiom-neural | 28 |        V7-D4: EmergentSubsystemRules { min_primitives, min_evidence_strength, require_review, max_active_candidates };
| axiom-neural | 28 |        ApproveError: NotFound / InvalidTransition / InsufficientEvidence / TooManyCandidates; 6 тестов
| axiom-neural | 28 |
| axiom-neural | 28 |FractalChain — N уровней AshtiCore (MAYA[n] → SUTRA[n+1], skills exchange)
| axiom-neural | 28 |ConfigWatcher — горячая перезагрузка axiom.yaml (inotify), передаётся в tick_loop
| axiom-neural | 28 |EventBus — pub/sub: типизированные и broadcast подписки
| axiom-neural | 28 |domain_name() — pub fn в axiom-runtime (EA-TD-01 ✅)
| axiom-neural | 28 |
| axiom-neural | 28 |axiom-agent:
| axiom-neural | 28 |  ├── TextPerceptor — текст → UclCommand(InjectToken): 2-path detect_subsystem()
| axiom-neural | 28 |  │     Path1: AnchorSet.match_text() + dominant_subsystem_of(); Path2: AnchorMatchTable.dominant_subsystem()
| axiom-neural | 28 |  │     (word_signals weight=1.0 + char_signals weight×0.4 → subsystem_from_anchor_id prefix map)
| axiom-neural | 28 |  │     100% per-text subsystem accuracy (OBS-02, 8 корпусов × 30k тиков)
| axiom-neural | 28 |  ├── L0VisionPerceptor ✅ (V7-E2) — RGBA8 → grayscale → Sobel edge detection → stroke classification;
| axiom-neural | 28 |  │     EdgeAnalysis { edge_density, horizontal_fraction, vertical_fraction, diagonal_fraction };
| axiom-neural | 28 |  │     InjectToken в SUTRA(100) для каждого L0 примитива с density ≥ 0.02;
| axiom-neural | 28 |  │     Anchors: visual_edge / stroke_horizontal / stroke_vertical / stroke_diagonal; 10 тестов
| axiom-neural | 28 |  ├── MessageEffector — ProcessingResult → диагностический вывод (DetailLevel: off/min/mid/max)
| axiom-neural | 28 |  ├── MLEngine (mock + ONNX) → VisionPerceptor (explicit ShapeMismatch при input_size=0),
| axiom-neural | 28 |  │   AudioPerceptor (VAD)
| axiom-neural | 28 |  ├── CLI Channel: stdin/stdout loop, axiom-cli.yaml, все :команды
| axiom-neural | 28 |  │   CLI Extended V1.0: :domain/:events/:frontier/:guardian/:watch/:config/:trace/:connections
| axiom-neural | 28 |  │   :dream/:multipass/:reflector/:impulses/:schema/:anchors/:match/:help/:perf/:tickrate
| axiom-neural | 28 |  │   Горячая перезагрузка config/axiom.yaml (--hot-reload) через ConfigWatcher → tick_loop
| axiom-neural | 28 |  │   domain config hot-reload: apply_domain_config() при watcher.poll()
| axiom-neural | 28 |  └── External Adapters (Phase 0–5 + Tech Debt Closure):
| axiom-neural | 28 |      ├── tick_loop — единственный writer AxiomEngine; CliState (PerfTracker, event_log,
| axiom-neural | 28 |      │              watch_fields, multipass_count); адаптивный sleep (EA-TD-03/04 ✅)
| axiom-neural | 28 |      │              Workstation commands: handle_wstation_command + RunBench с BenchProgress events
| axiom-neural | 28 |      ├── AdapterCommand / AdapterPayload — Inject, MetaRead, MetaMutate, DomainSnapshot,
| axiom-neural | 28 |      │              Subscribe, Unsubscribe; AdapterSource: Cli, WebSocket, Rest, Telegram
| axiom-neural | 28 |      ├── ServerMessage — Result, Tick, State, CommandResult, DomainDetail, Error (serde JSON)
| axiom-neural | 28 |      ├── WebSocket (Phase 1) — axum 0.8/ws, /ws endpoint, подписки ticks/state,
| axiom-neural | 28 |      │              --server / --port флаги; AppState shared через Arc
| axiom-neural | 28 |      ├── REST (Phase 2) — axum Router, 5 handlers (inject/status/domains/traces/domain-detail),
| axiom-neural | 28 |      │              корреляция через broadcast + timeout 5s
| axiom-neural | 28 |      ├── Dashboard (Phase 3) — tools/axiom-dashboard, egui/eframe, sync tungstenite,
| axiom-neural | 28 |      │              4 панели: Status, Space View, Domain List, Input
| axiom-neural | 28 |      ├── Telegram (Phase 4, feature "telegram") — long-poll getUpdates, route_message,
| axiom-neural | 28 |      │              pending HashMap корреляция, --telegram-token / --telegram-allow
| axiom-neural | 28 |      └── OpenSearch (Phase 5, feature "opensearch") — индексирует Result+Tick events,
| axiom-neural | 28 |                     build_result_doc / build_tick_doc, fire-and-forget POST,
| axiom-neural | 28 |                     --opensearch-url / --opensearch-index / --opensearch-tick
| axiom-neural | 28 |
| axiom-neural | 28 |axiom-runtime:
| axiom-neural | 28 |  ├── process_and_observe() — обёртка process_command() с диагностикой (ProcessingResult)
| axiom-neural | 28 |  ├── Orchestrator — параллельная маршрутизация + Guardian check + apply_feedback
| axiom-neural | 28 |  │   route_token_limited (S5): routing через роли 1–N вместо 1–8
| axiom-neural | 28 |  ├── AdaptiveTickRate — Variable Tick Rate (min_hz=60, max_hz=1000, cooldown=50)
| axiom-neural | 28 |  ├── domain_name(id: u16) — pub fn, экспортируется без feature-gate
| axiom-neural | 28 |  ├── Axiom Sentinel V1.1 ✅ (2026-05-12):
| axiom-neural | 28 |  │   S0: thread_pool → Arc<rayon::ThreadPool> в global OnceLock; AxiomEngine::new < 800 µs
| axiom-neural | 28 |  │   S1: inject_token_direct — bypass UCL-парсинга; ~20 ns vs ~35 ns для сенсорных данных
| axiom-neural | 28 |  │   S2: Experience::set_max_traces / should_trigger_export (×5000) / estimate_memory_bytes;
| axiom-neural | 28 |  │       TickSchedule::memory_pressure_threshold_bytes (1.8 GiB) → немедленный horizon GC
| axiom-neural | 28 |  │   S3: apply_gravity_batch_chunked + L2_CHUNK_TOKENS=65536 (512 KB / 8 B per token)
| axiom-neural | 28 |  │   S4: .cargo/config.toml target-cpu=native → авто-векторизация AVX2 в release/bench
| axiom-neural | 28 |  │   S4b: apply_gravity_batch_avx2 — явные AVX2 intrinsics (VSQRTPS+VDIVPS), 8 tok/iter;
| axiom-neural | 28 |  │        6.74 ms @ 1M токенов (цель 8–10 ms ✅); early exit shift≥16; scalar fallback
| axiom-neural | 28 |  │   S5: TickBudget (tick_budget_start / budget_used_fraction); enable_layer_priority gate;
| axiom-neural | 28 |  │       при budget>80% роли 4–8 пропускаются (process_parallel_limited / route_token_limited)
| axiom-neural | 28 |  │   S6: prepare_speculative_grids(pool) — параллельная pre-build SpatialHashGrid для reconcile_all;
| axiom-neural | 28 |  │       speculative_grids[11] + hits/misses счётчики; ~9 µs swap vs ~40 µs rebuild при hit ✅
| axiom-neural | 28 |  ├── TickSchedule: enable_layer_priority, target_tick_ns, memory_pressure_threshold_bytes
| axiom-neural | 28 |  ├── Over-Domain Layer (over_domain/): OverDomainComponent, Weaver traits; FrameWeaver V1.3
| axiom-neural | 28 |  │   BondTokens + ReinforceFrame + InjectFrameAnchor + UnfoldFrame handlers в engine.rs
| axiom-neural | 28 |  │   restore_frame_from_anchor (pub fn, over_domain::weavers::frame)
| axiom-neural | 28 |  └── Broadcast types (--features adapters): BroadcastSnapshot, DomainSummary,
| axiom-neural | 28 |      DomainDetailSnapshot, TokenSnapshot, ConnectionSnapshot; snapshot_for_broadcast(),
| axiom-neural | 28 |      domain_detail_snapshot(), trace_count(), tension_count(), last_matched()
| axiom-neural | 28 |
| axiom-neural | 28 |axiom-config (Config V1.0 + D-07 + Anchor V1.0 + DreamConfig):
| axiom-neural | 28 |  ├── PresetsConfig.heartbeat_file / dream_file → LoadedAxiomConfig.heartbeat / dream (Option<…>)
| axiom-neural | 28 |  ├── DreamConfig: SchedulerConfig + FatigueWeightsConfig + CycleConfig; default/dev/production/validate()
| axiom-neural | 28 |  ├── ConfigWatcher — поллится в tick_loop каждый тик (EA-TD-05 ✅)
| axiom-neural | 28 |  ├── schema — JsonSchema на всех конфигах включая DreamConfig, validate_yaml<T>(), :schema CLI-команда
| axiom-neural | 28 |  ├── AnchorSet — якорные токены: axes/layers/domains, YAML-загрузка, match_text(), compute_position/shell/weight;
| axiom-neural | 28 |  │     SUBSYSTEM_NAMES: [&str; 6], dominant_subsystem_of(matches) → Option<SubsystemId>
| axiom-neural | 28 |  ├── SubsystemDependencies ✅ — загрузчик §2.7 Variant C+ из config/subsystem_dependencies.yaml;
| axiom-neural | 28 |  │     SubsystemDep { builds_on, natural_tensions }, NaturalTension { target, reason };
| axiom-neural | 28 |  │     load_or_empty(config_dir) — graceful degradation; is_natural_tension(a,b) — симметрично;
| axiom-neural | 28 |  │     load_order() → topological sort (DFS), Err(String) при обнаружении цикла
| axiom-neural | 28 |  └── AnchorLayer ✅ (V7-A2) — L0/L1 флаг в Anchor; AnchorSet.perceptual: Vec<Anchor>;
| axiom-neural | 28 |        load_perceptual() из config/anchors/perceptual/ (graceful degradation);
| axiom-neural | 28 |        perceptual_anchors() accessor; L0 НЕ в match_text() (только VisionPerceptor);
| axiom-neural | 28 |        total_count() включает perceptual; 7 новых тестов
| axiom-neural | 28 |
| axiom-neural | 28 |axiom-persist (D-04):
| axiom-neural | 28 |  ├── save/load: Token+Connection+ExperienceTrace → bincode (атомарный rename)
| axiom-neural | 28 |  ├── MemoryManifest (YAML), IMPORT_WEIGHT_FACTOR=0.7
| axiom-neural | 28 |  ├── AutoSaver: интервальное автосохранение, force_save при :quit
| axiom-neural | 28 |  ├── exchange: export/import traces+skills (bincode), GUARDIAN-валидация
| axiom-neural | 28 |  ├── ARB-TD-05: StoredTrustEntry в StoredEngineState; save→export_trust_calibration(); load→import_trust_calibration()
| axiom-neural | 28 |  │     TrustConfig: iter_entries() + set_min_confidence() (pub API для сериализации)
| axiom-neural | 28 |  └── ARB-TD-06: octant_weights: Option<[f32;8]> в StoredEngineState; save→cognitive_profile().octant_weights;
| axiom-neural | 28 |        load→CognitiveProfile::with_weights(weights) (с клампингом); 2 новых roundtrip-теста
| axiom-neural | 28 |
| axiom-neural | 28 |axiom-space:
| axiom-neural | 28 |  ├── apply_gravity_batch — scalar, детерминировано точный (feature "simd")
| axiom-neural | 28 |  ├── apply_gravity_batch_avx2 — AVX2 f32, Linear, 8 tok/iter; 6.74 ms@1M (S4b ✅)
| axiom-neural | 28 |  └── apply_gravity_batch_chunked + L2_CHUNK_TOKENS — L2-cache-friendly batch для N>1M (S3)
| axiom-neural | 28 |
| axiom-neural | 28 |axiom-node HTTP ✅ (2026-05-29):
| axiom-neural | 28 |  axum HTTP-сервер на :8080; маршруты:
| axiom-neural | 28 |    GET  /api/ws                    — WebSocket JSON bridge (snapshot при подключении + EngineEvent)
| axiom-neural | 28 |    POST /api/advisory/confirm/{id} — NodeCmd::AdvisoryConfirm → engine.confirm_pending_advisory()
| axiom-neural | 28 |    POST /api/advisory/reject/{id}  — NodeCmd::AdvisoryReject → engine.reject_pending_advisory()
| axiom-neural | 28 |    POST /api/text/submit           — NodeCmd::SubmitText → perceptor.perceive() → engine
| axiom-neural | 28 |    GET  /metrics                   — Prometheus text format (~30 метрик)
| axiom-neural | 28 |    POST /api/lab/run               — запустить lab job (obs/bench_*/test/showcase)
| axiom-neural | 28 |    POST /api/lab/stop              — остановить текущий job
| axiom-neural | 28 |    GET  /api/lab/status            — статус текущего job (JSON)
| axiom-neural | 28 |    GET  /api/lab/ws/log            — WebSocket stream stdout/stderr текущего job
| axiom-neural | 28 |    GET  *                          — ServeDir(web_dist) статика Workstation V2
| axiom-neural | 28 |  NodeCmd channel: unbounded mpsc HTTP→tick_loop; нет Mutex на AxiomEngine
| axiom-neural | 28 |  BroadcastHandle: subscribe_events() → Receiver<EngineMessage>; latest_snapshot() → Option<SystemSnapshot>;
| axiom-neural | 28 |    snapshot_live: RwLock<Option<SystemSnapshot>> — хранит живой снапшот для /metrics и WS bridge
| axiom-neural | 28 |
| axiom-neural | 28 |Workstation V2.0 ✅ (2026-05-24):
| axiom-neural | 28 |  axiom-web — React 18 SPA + Zustand + Vite (tools/axiom-web/):
| axiom-neural | 28 |    8 табов: Overview / Domains / Traces / Internals / Conversation / Phase C / Patterns / Lab
| axiom-neural | 28 |  Advisory Queue: confirm/reject → POST /api/advisory/confirm|reject/{id}, TTL bar
| axiom-neural | 28 |  SVG sparklines (zero-dep, rolling 120 snapshots), domain activity grid
| axiom-neural | 28 |  Авто-переподключение WS каждые 2s; badge на Phase C tab при pending advisories
| axiom-neural | 28 |  Grafana + Prometheus (tools/grafana/docker-compose.yml): 3 provisioned дашборда, scrape 5s
| axiom-neural | 28 |
| axiom-neural | 28 |Lab панель ✅ (2026-05-29):
| axiom-neural | 28 |  axiom-node/src/lab.rs: POST /api/lab/run|stop, GET /api/lab/status, GET /api/lab/ws/log
| axiom-neural | 28 |  Запуск OBS / Hot Bench / OverDomain Bench / Stress / Tests / Full Showcase из браузера
| axiom-neural | 28 |  Прогресс-бар OBS (парсинг [observe] N/M (%)), цветной лог, Results panel, история прогонов
| axiom-neural | 28 |
| axiom-neural | 28 |Performance & Tooling Sprint ✅ (2026-05-29):
| axiom-neural | 28 |  Token lifecycle: check_decay → TokenDecayed → STATE_SLEEPING (valence=0); scan_region
| axiom-neural | 28 |    пропускает спящие токены; add_token вытесняет спящие при переполнении; eviction hook → Experience
| axiom-neural | 28 |  Parallel ticks: AshtiCore::tick() — sequential heartbeat + parallel process_frontier (rayon)
| axiom-neural | 28 |  Parallel OBS shards: N AxiomEngine на N потоках; corpus_large.yaml: shards=4
| axiom-neural | 28 |  OBS streaming: run_streaming() → snapshots.jsonl + events.jsonl (BufWriter, RAM flat)
| axiom-neural | 28 |  corpus_showcase.yaml: 18 текстов, 9 подсистем, 200K тиков, ~3-5 мин
| axiom-neural | 28 |  corpus_profile.yaml: 4 текста, 50K тиков — для cargo flamegraph профилирования
| axiom-neural | 28 |  INVARIANTS.md v11: правило о жизненном цикле токенов (не удаляются, → STATE_SLEEPING)
| axiom-neural | 28 |
| axiom-neural | 28 |```
| axiom-neural | 28 |
| axiom-neural | 28 |**Документация:** [docs/guides/AXIOM_GUIDE.md](docs/guides/AXIOM_GUIDE.md)
| axiom-neural | 28 |
| axiom-neural | 28 |---
| axiom-neural | 28 |
| axiom-neural | 28 |## Crates
| axiom-neural | 28 |
| axiom-neural | 28 || Crate | Тесты | Описание |
| axiom-neural | 28 ||-------|-------|----------|
| axiom-neural | 28 || axiom-core | 34 | Token, Connection, Event |
| axiom-neural | 28 || axiom-genome | 26 | Genome V1.0: конституция, GenomeIndex, from_yaml; ModuleId=22 (Waves), MAX_MODULES=23; EmergentSubsystemRules (V7-D4); CrossModalConfig (CMB-TD-02); MembraneProfile (Domain_Membrane_Profiles_V1_0) |
| axiom-neural | 28 || axiom-frontier | 32 | CausalFrontier V2.0, Storm Control, BatchToken/BatchConnection, budget |
| axiom-neural | 28 || axiom-config | 115 | DomainConfig, ConfigLoader, YAML presets, ConfigWatcher, HeartbeatConfig, DreamConfig, JsonSchema, AnchorSet; SubsystemDependencies; AnchorLayer L0/L1; perceptual_anchors() |
| axiom-neural | 28 || axiom-space | 118 | SpatialHashGrid, физика, apply_gravity_batch, apply_gravity_batch_avx2 (AVX2, feature "simd", S4b) |
| axiom-neural | 28 || axiom-shell | 48 | Shell V3.0, семантические профили, from_yaml; link_types: 0x08 Syntactic, 0x09 Composition, 0x0A CrossModal, 0x0B SemanticAnchor=0x0B01 (AE-TD-08) |
| axiom-neural | 28 || axiom-arbiter | 154 | Arbiter V1.0 + membrane_profiles/blend_factor + configure_membranes(); membrane_transform() (blend_u8/i8+clamp); route_to_ashti: membrane перед process_token (slow path only); Experience (shell_registry: HashMap<u32,[u8;8]>; shell_cosine() → 15% бонус; Shell-TD-02), REFLECTOR, SKILLSET, GridHash, AshtiProcessor, COM |
| axiom-neural | 28 || axiom-heartbeat | 15 | Heartbeat V2.0 |
| axiom-neural | 28 || axiom-upo | 13 | UPO v2.2: DynamicTrace, Screen, UPO::compute |
| axiom-neural | 28 || axiom-ucl | 9 | UCL commands |
| axiom-neural | 28 || axiom-domain | 126 | Domain, DomainState, AshtiCore, CausalHorizon, FractalChain, Speculative Layer (S6) |
| axiom-neural | 28 || axiom-experience | 50 | AxialStore, SutraDepthStore, InterpretationProfileStore, EmergentPrimitiveStore, MetaStore; FatigueStore + SubsystemFatigue (V7-B2); ModalityStore + Modality (Text/Vision/Internal); Octant (8), SubsystemId (+Morality/Abstractions/Dilemmas), EvaluationLevel |
| axiom-neural | 28 || axiom-neural | 26 | Neural Integration Этап 1: ReactivationDepthModel (1D-CNN Conv1D(9→32,k=3)→Conv1D(32→64,k=5)→GAP→Linear(64→32)→Linear(32→8/1), INPUT_SIZE=1539, ~13K params); FftFrontend (rustfft, static scratch); ConfidenceCalibrator; AdvisorMode {Rule, Neural}; ReactivationDepthConfig from_arch(); load/save .bin (bincode); нет alloc в infer() |
| axiom-neural | 28 || axiom-runtime | 665 (684 features adapters) | AxiomEngine, Guardian, Over-Domain Layer (FrameWeaver V1.3, AxialEvaluator V3.0, ContextRecognizer V6.0+V7, **NeuralAdvisor V3.0** + NeuralReactivationDepthAdvisor (mode=Rule/Neural, 1ms timeout, fallback), OverDomainArbiter V3.0, **Sensorium V2.0**, **Waves V1.0**), DREAM Phase V1.1, Gateway, Channel, EventBus, TickSchedule (+subsystem_gravity_interval=500), **SubsystemGravityRule** (PRIM-TD-03); **SEN-TD-01**: BroadcastSnapshot удалён, last_dream_summary pub, SensoriumState единственный runtime-пульс; broadcast.rs: LastDreamSummary+DomainDetailSnapshot+TokenSnapshot+ConnectionSnapshot; subsystem_gravity.rs; inject_anchor_tokens → set_shell_registry (Shell-TD-02) |
| axiom-neural | 28 || axiom-agent | 164 (187 telegram,opensearch) | TextPerceptor (2-path detect_subsystem + perceive_and_bond→SEMANTIC_ANCHOR_BOND=0x0B01; text_stable_id 0x4000_0001+; anchor_sutra_id mirror); AnchorMatchTable: domain+layer якоря в id_to_position (P4b); L0VisionPerceptor (V7-E2); MessageEffector, CliChannel + CLI Extended V1.0 + Anchor commands; tick_loop (CliState, adaptive sleep, ConfigWatcher, domain hot-reload, RunBench), AdapterCommand, ServerMessage; External Adapters Phase 0–5; Telegram (feature), OpenSearch (feature) |
| axiom-neural | 28 || axiom-persist | 37 | MemoryWriter, MemoryLoader, MemoryManifest, AutoSaver, exchange (bincode); ARB-TD-05 TrustConfig calibration roundtrip; ARB-TD-06 CognitiveProfile octant_weights roundtrip |
| axiom-neural | 28 || axiom-protocol | 41 | EngineCommand(15)/Event/Message, SystemSnapshot+TokenFieldPoint, ConfigSchema, BenchSpec, AdapterInfo, FrameWeaverStats(syntactic_layer_activations); postcard round-trip; WS-5: +PerfSnapshot, TraceSnapshot, TensionTraceSnapshot, ReflectorSnapshot, CognitiveDepthSnapshot, ImpulsesSnapshot; SystemSnapshot: +perf/traces/tension/reflector/cognitive_depth/impulses/skills_count |
| axiom-neural | 28 || axiom-broadcasting | 7 | BroadcastServer, BroadcastHandle (sensorium_live: RwLock<Option<String>>, update_sensorium(), latest_sensorium_json()), subscription filter, heartbeat (BRD-TD-06: pong timeout test через raw TCP), build_system_snapshot (прямые запросы к &AxiomEngine, без BroadcastSnapshot); BroadcastSnapshot удалён (SEN-TD-01 Фаза F) |
| axiom-neural | 28 || axiom-node | — | HTTP-сервер (axum): WS JSON bridge, advisory confirm/reject, /metrics, ServeDir; NodeCmd channel; tick_loop интеграция; WS-5: NodePerfTracker (window=100) → PerfSnapshot per snapshot |
| axiom-neural | 28 || tools/axiom-web | — | React 18 SPA: Overview/Conversation/Phase C/Patterns; AdvisoryQueue, Sparklines, Zustand store; WS-5: protocol.ts extended with PerfSnapshot/TraceSnapshot/TensionTraceSnapshot/ReflectorSnapshot/CognitiveDepthSnapshot/ImpulsesSnapshot |
| axiom-neural | 28 |
| axiom-neural | 28 || axiom-bench | — | Criterion бенчмарки (результаты: `docs/bench/RESULTS.md`) |
| axiom-neural | 28 || axiom-corpus | 4 | Corpus loader: 8 текстовых корпусов для OBS-прогонов |
| axiom-neural | 28 || tools/axiom-dashboard | 6 | egui/eframe Desktop GUI — Status, Space View, Domain List, Input panels |
| axiom-neural | 28 || tools/axiom-tray | 6 | Системный трей (ksni): StatusNotifierItem, poll /metrics каждые 2s, Start/Stop axiom-node, Open Workstation |
| axiom-neural | 28 || axiom-observe | — | ObsRunner, OBS-01: автоматизация прогонов, MetricsCollector, report.md; **training_data.jsonl** (каждые 200 тиков: one-hot кольца × 9 подсистем + teacher reactivation_weights[8]) |
| axiom-neural | 28 || **Итого** | **1536** (all features) + TEST-TD-01 (DEFERRED) | |
| axiom-neural | 28 |
| axiom-neural | 28 |---
| axiom-neural | 28 |
| axiom-neural | 28 |## Этапы
| axiom-neural | 28 |
| axiom-neural | 28 || Этап | Название | Статус |
| axiom-neural | 28 ||------|----------|--------|
| axiom-neural | 28 || 1 | GENOME + GUARDIAN | ✅ |
| axiom-neural | 28 || 2 | Storm Control | ✅ |
| axiom-neural | 28 || 3 | Configuration System | ✅ |
| axiom-neural | 28 || 4 | REFLECTOR + SKILLSET | ✅ |
| axiom-neural | 28 || 5 | GridHash-индекс | ✅ |
| axiom-neural | 28 || 6 | Адаптивные пороги | ✅ |
| axiom-neural | 28 || 7 | Causal Horizon + Масштабирование | ✅ |
| axiom-neural | 28 || 8 | External Integration Layer | ✅ |
| axiom-neural | 28 || 9 | Tech Debt + EventBus + Config hot reload | ✅ |
| axiom-neural | 28 || 10 | Agent Layer (CLI/Telegram/Shell) | ✅ |
| axiom-neural | 28 || 11 | ML Inference | ✅ |
| axiom-neural | 28 || 12 | FractalChain + SIMD-физика | ✅ |
| axiom-neural | 28 || 13A | Cognitive Depth — Multi-pass + TensionTrace | ✅ |
| axiom-neural | 28 || 13B | Cognitive Depth — Heartbeat Internal Drive | ✅ |
| axiom-neural | 28 || 13C | Cognitive Depth — InternalImpulse + Dominance | ✅ |
| axiom-neural | 28 || 13D | Cognitive Depth — Goal Persistence + Curiosity | ✅ |
| axiom-neural | 28 || Cleanup | COM V1.1 — unwrap, Unknown, Event fields, COM, constants, TickSchedule | ✅ |
| axiom-neural | 28 || Memory | Memory Persistence V1.0 — save/load/autosave/exchange (axiom-persist) | ✅ |
| axiom-neural | 28 || CLI V1.1 | CLI Channel V1.1 — TextPerceptor, MessageEffector, process_and_observe, axiom-cli.yaml | ✅ |
| axiom-neural | 28 || Sentinel | Axiom Sentinel V1.0 — Hardware Topology, Parallel Resonance Search, Variable Tick Rate | ✅ |
| axiom-neural | 28 || CLI Ext | CLI Extended V1.0 (Phase 1-3) — 13 новых команд, detail levels, multipass tracker | ✅ |
| axiom-neural | 28 || Config | Config V1.0 — HeartbeatConfig load, ConfigWatcher→tick_loop, hot_reload | ✅ |
| axiom-neural | 28 || D-04 | axiom-persist: bincode вместо serde_json (3-5× меньше, 2-4× быстрее) | ✅ |
| axiom-neural | 28 || D-07 | JSON-schema валидация конфигов — schemars + jsonschema, :schema CLI-команда | ✅ |
| axiom-neural | 28 || Anchor | Anchor Tokens V1.0 (Phase 1-3) — AnchorSet, YAML, TextPerceptor, inject_anchor_tokens в SUTRA+домены, :anchors/:match | ✅ |
| axiom-neural | 28 || Adapters 0A | BroadcastSnapshot + convenience methods (axiom-runtime --features adapters) | ✅ |
| axiom-neural | 28 || Adapters 0B | Рефактор handle_meta_command → handle_meta_read / handle_meta_mutate | ✅ |
| axiom-neural | 28 || Adapters 0C | tick_loop, AdapterCommand, ServerMessage, AdaptersConfig; CLI → тонкая обёртка | ✅ |
| axiom-neural | 28 || Adapters 1 | WebSocket адаптер — axum 0.8, /ws, подписки, --server / --port | ✅ |
| axiom-neural | 28 || Adapters 2 | REST адаптер — axum Router, 5 handlers, correlation broadcast+timeout | ✅ |
| axiom-neural | 28 || Adapters 3 | egui Dashboard — tools/axiom-dashboard, sync WS client, 4 панели | ✅ |
| axiom-neural | 28 || Adapters 4 | Telegram адаптер — long-poll, route_message, pending корреляция | ✅ |
| axiom-neural | 28 || Adapters 5 | OpenSearch адаптер — Result+Tick indexing, fire-and-forget POST | ✅ |
| axiom-neural | 28 || Tech Debt | EA-TD-01..06: domain_name, CliState, adaptive tick, ConfigWatcher, DetailLevel | ✅ |
| axiom-neural | 28 || EA-TD-02 | TokenSnapshot::shell — точный compute_shell через SemanticContributionTable | ✅ |
| axiom-neural | 28 || FrameWeaver 1–3 | Over-Domain Layer traits + FrameWeaver V1.1 (scan→EXPERIENCE, ReinforceFrame, CycleStrategy::Allow) | ✅ |
| axiom-neural | 28 || FrameWeaver 4 | Интеграция в AxiomEngine (on_tick + drain_commands), BroadcastSnapshot + FrameWeaverStats, GENOME permissions | ✅ |
| axiom-neural | 28 || FrameWeaver 5 | 26 unit-тестов: fnv1a, scan, crystallization, reactivation, promotion, stats | ✅ |
| axiom-neural | 28 || FW Stabilization | E1: restore_frame_from_anchor + UnfoldFrame handler + реальная промоция; E2: tick в Weaver trait; E3: drain_commands оптимизация 311→238 ns; E4 deferred. | ✅ |
| axiom-neural | 28 || FrameWeaver V1.2 | Промоция перенесена из on_tick (steps 4–5) → dream_propose(); вызов при FallingAsleep; Errata E2–E4 зафиксированы | ✅ |
| axiom-neural | 28 || DREAM Phase 1–5 | DreamScheduler + FatigueTracker + DreamCycle + DreamProposal + GUARDIAN check_frame_anchor_sutra_write; unit-тесты | ✅ |
| axiom-neural | 28 || DREAM Phase 6 | CLI :dream-stats / :force-sleep / :wake-up; BroadcastSnapshot расширен; dream_cli_tests (5 тестов) | ✅ |
| axiom-neural | 28 || DREAM Phase 7 | Smoke-тест 8 тестов: full_cycle, multiple_cycles, interrupted_cycle, scheduler_stats, promotions | ✅ |
| axiom-neural | 28 || DreamConfig | axiom-config: SchedulerConfig+FatigueWeightsConfig+CycleConfig; apply_dream_config() в engine; Gateway::with_config(); hot-reload; dream.yaml; :schema dream | ✅ |
| axiom-neural | 28 || WS Stage 0–1 | axiom-protocol (41 тест) + axiom-broadcasting scaffold; postcard сериализация | ✅ |
| axiom-neural | 28 || WS Stage 2 | axiom-broadcasting: BroadcastServer/Handle, filter, heartbeat, 6 тестов | ✅ |
| axiom-neural | 28 |
| axiom-neural | 28 || Protocol C1–C3 | syntactic_layer_activations [u8;8] в FrameWeaverStats (C1); RunBench в протоколе + tick_loop (C2); TokenFieldPoint + token_field в DomainSnapshot + Live Field real data (C3) | ✅ |
| axiom-neural | 28 || Engine D1–D6 | tick в check_promotion (D1); min_participant_anchors cross-domain (D2); все RuleTrigger (D3); GENOME on_boot enforcement (D4); domain config hot-reload apply_domain_config (D5); domain_activity_threshold + Lagged resync (D6) | ✅ |
| axiom-neural | 28 || E2 | MLEngine size check — явная ShapeMismatch вместо silent fallback (D-06 закрыт) | ✅ |
| axiom-neural | 28 || Phase C1 | axiom-experience: AxialStore, SutraDepthStore, InterpretationProfileStore, EmergentPrimitiveStore; Octant×8 | ✅ |
| axiom-neural | 28 || Phase C2 | AnchorSet: subsystem architecture, writing/mathematics primitives, FlatAnchorFile YAML | ✅ |
| axiom-neural | 28 || Phase C3 | AxialEvaluator V1.0 (ModuleId=17, tick=5): X/Y/Z axes, 8 EvaluationLevels, Corpus Callosum conflict | ✅ |
| axiom-neural | 28 || Phase C6 | AxialEvaluator V2.0: subsystem-aware level selection, OctantStabilityTracker, ConflictPersistenceTracker, drain_pending_advisories, AXIAL_EVALUATOR_SOURCE_ID=1; TrustConfig расширен source=1 | ✅ |
| axiom-neural | 28 || Phase C4 | ContextRecognizer V1.0 (ModuleId=18, tick=7): ScanningPlan, SubsystemEnergy, InterpretationProfile | ✅ |
| axiom-neural | 28 || Phase C5 | NeuralAdvisor V1.0 (ModuleId=19, tick=11): advisory-only, 5 трейтов, RuleBasedCorpusCallosumResolver, DepthThresholdEmergentDetector; on_tick → Vec<UclCommand> | ✅ |
| axiom-neural | 28 || Phase I1 | Engine coordinator: axial_evaluator/context_recognizer/neural_advisor — конкретные поля AxiomEngine, tick % 5/7/11, snapshot sync AE→CR→NA; opcode_from_u16 расширен; 9 тестов | ✅ |
| axiom-neural | 28 || Phase I4 | ApproveEmergentCandidate (UCL 5201) handler в Engine → neural_advisor.approve_emergent(sutra_id) | ✅ |
| axiom-neural | 28 || Phase I2 | ContextRecognizer::from_anchor_set(AnchorSet): build_subsystem_refs по именам подсистем; AxiomEngine::apply_anchor_set; axiom-node/startup вызывает при старте; 3 теста | ✅ |
| axiom-neural | 28 || Phase I3 | Якорный контент: config/anchors/writing/primitives.yaml (7 графических примитивов) + config/anchors/mathematics/primitives.yaml (7 структурных примитивов); ContextRecognizer подхватывает через get_subsystem(); integration test в anchor.rs | ✅ |
| axiom-neural | 28 || Phase I6 | Workstation Phase C visibility: PhaseCSnapshot в SystemSnapshot (dominant_octant/subsystem, emergent_candidates); ApproveEmergentCandidate в EngineCommand + axiom-node handler; Patterns tab — Phase C panel (октант+подсистема с цветом, emergent candidates с кнопкой Approve) | ✅ |
| axiom-neural | 28 || Phase I7 | OverDomainArbiter V1.0 (ModuleId=20, tick=13): AdvisorySource трейт, TrustConfig, PendingQueue, ArbiterLog; NeuralAdvisor реализует AdvisorySource; on_boot в try_new; PhaseCSnapshot расширен (octant_depth_avg, pending_advisories); Workstation: octant depth panel + arbiter queue panel; три DepthHint советника: ReactivationDepth, SubsystemAffinity, AgeDecay(DEPTH_FLOOR=50) | ✅ |
| axiom-neural | 28 || CR-V6 Фаза 0 | SyntacticBridge: bridge_to_maya + domain_position_hash в orchestrator.rs; MAYA получает 8 0x08-связей на каждый routing; FrameWeaver кристаллизует Frame-анкеры; 2 integration-теста | ✅ |
| axiom-neural | 28 || CR-V6 Фаза A | ActivityTrace (3 кольцевых буфера short=16/mid=64/long=256), ActivityDynamics (4 метрики), ActivitySignature classifier (6 сигнатур, приоритет Steady→Oscillating→Cascading→Converging→Diverging), ActivityAnalyzer (переименован из TransitionDetector); 15 unit-тестов | ✅ |
| axiom-neural | 28 || CR-V6 Фаза B | SubsystemFatigue { activation_load, recovery_debt }, FatigueStore; decay=0.90/tick, equilibrium=10.0; DREAM: activation_load *= 0.35; apply_to_weights() снижает вес уставших подсистем; 12 unit-тестов + integration | ✅ |
| axiom-neural | 28 || TextPerceptor 2-path | detect_subsystem(): Path1=AnchorSet.match_text()+dominant_subsystem_of(), Path2=AnchorMatchTable.dominant_subsystem(); word_signals+char_signals×0.4; subsystem_from_anchor_id prefix map; AnchorSet.SUBSYSTEM_NAMES + dominant_subsystem_of() | ✅ |
| axiom-neural | 28 || OBS-02 | Автоматизированный прогон: 30k тиков, 8 корпусных текстов, 415 инъекций, 100% per-text accuracy (исправлен "каждый" в logic_quantifier). 312 emergent-кандидатов (все Frame). SutraDepthStore reactivation_count: мёртвое поле исправлено (инкремент при apply_evidence с evidence>0). Пороги DepthThresholdEmergentDetector: MIN_DEPTH 8000→1000, MIN_REACTIVATIONS 30→5 (откалибровано по O7 avg_depth=1198, ~10-15 DREAM-циклов за 30k тиков) | ✅ |
| axiom-neural | 28 || OBS-infra | FrameCandidate.shell_similarity: f32; FrameWeaver.avg_candidate_shell_similarity(); ContextRecognizer.compute_raw_energies(); AxiomEngine.snapshot_subsystem_energies() — диагностическая инфраструктура для OBS-снимков | ✅ |
| axiom-neural | 28 || NeuralAdvisor V2.0 | Все 5 слотов заполнены: DepthHistoryBiasAdvisor (octant), AnchorVotingAdvisor (subsystem); AdvisoryHistory ring-32; OctantAdvisorInput+depth_per_octant/reactivation_count; CognitiveProfile octant_weights[8] в Arbiter с online learning rate=0.05; Advisory.octant_hint: Option<usize>; engine → with_default_v2() | ✅ |
| axiom-neural | 28 || AxialEvaluator V3.0 | NarrativeOctantTracker (advisory override), adaptive stability threshold, AxialStore::override_octant(sutra_id, octant) | ✅ |
| axiom-neural | 28 || OverDomainArbiter V2.0 | TrustConfig from yaml (genome.yaml [arbiter.trust]); TTL 1000 (expires_at_event); CognitiveProfile from yaml (balanced/analytic); confirm/reject_pending_advisory в AxiomEngine | ✅ |
| axiom-neural | 28 || WS-0 | axiom-node: axum HTTP + WS JSON bridge; React scaffold; axiom-broadcasting: subscribe_events, latest_snapshot | ✅ |
| axiom-neural | 28 || WS-1 | Advisory Queue UI: confirm/reject кнопки + TTL bar; REST endpoints advisory/confirm|reject/{id} | ✅ |
| axiom-neural | 28 || WS-2 | Core Tabs: Conversation (feed + textarea), Phase C (octant depth, emergent, advisory), Patterns (sparklines L1–L8, domain grid) | ✅ |
| axiom-neural | 28 || WS-3 | /metrics Prometheus endpoint (~30 метрик); tools/grafana: docker-compose, 3 provisioned дашборда | ✅ |
| axiom-neural | 28 || ARB-TD-05/06 | axiom-persist: persist TrustConfig calibration (StoredTrustEntry) + CognitiveProfile octant_weights; TrustConfig: iter_entries()+set_min_confidence(); loader restores both; 2 roundtrip tests | ✅ |
| axiom-neural | 28 || Phase G1 | NeuralAdvisor V3.0: DivergenceLog (ring 256) — расхождения advisory_octant ↔ analytic_octant (Hamming ≥ 2); octant_hamming_distance() | ✅ |
| axiom-neural | 28 || Phase G2 | NeuralAdvisor V3.0: PatternLearningResolver (conflict slot) — online learning на AdvisoryHistory per-Frame | ✅ |
| axiom-neural | 28 || Phase G3 | NeuralAdvisor V3.0: NeuralAdvisorConfig — genome.yaml секция neural_advisor → per-advisor enable/disable | ✅ |
| axiom-neural | 28 || OverDomainArbiter V3.0 | drain_octant_overrides() → pending overrides для AxialEvaluatorStorage; feedback-буфер для незарегистрированных источников | ✅ |
| axiom-neural | 28 || WS-5 | axiom-node: NodePerfTracker → PerfSnapshot; SystemSnapshot расширен (traces/tension/reflector/cognitive_depth/impulses/skills); React SPA: Domains, Traces, Internals tabs + расширенный Overview | ✅ |
| axiom-neural | 28 || Phase H1 | DREAM Phase V1.1: cluster_emergent_primitives() → SubsystemCandidateStore; NotifySubsystemCandidate (UCL 5300) | ✅ |
| axiom-neural | 28 || Phase H2 | DREAM Phase V1.1: SubsystemLifecycleState (Proposed→Candidate→InReview→Active→Mature→Deprecated→Archived); ApproveSubsystemCandidate (UCL 5301) | ✅ |
| axiom-neural | 28 || WS-6 | axiom-tray: системный трей (ksni), poll /metrics каждые 2s, Start/Stop axiom-node, Open Workstation в браузере | ✅ |
| axiom-neural | 28 || Primitive YAMLs | config/anchors/morality/primitives.yaml (7 Haidt: moral_care..moral_desecration, Shell L1/L4/L6); config/anchors/abstractions/primitives.yaml (7 мета-якорей A0–A6, C0→C5+, temp 3–9); config/anchors/time/primitives.yaml (T1–T7: time_before..time_horizon); config/anchors/values/primitives.yaml (V1–V7: val_beneficial..val_forbidden) — выровнены со спецификациями | ✅ |
| axiom-neural | 28 || config/subsystem_dependencies.yaml | §2.7 Variant C+: 7 подсистем (writing/mathematics/time/morality/values/abstractions/dilemmas), builds_on + natural_tensions | ✅ |
| axiom-neural | 28 || SubsystemDependencies loader | axiom-config: SubsystemDependencies, SubsystemDep, NaturalTension; load_or_empty, is_natural_tension (симметрично), load_order() топо-сорт с детектированием цикла; 7 тестов | ✅ |
| axiom-neural | 28 || DilemmaStore V1.1 | axiom-runtime: DilemmaStore (max 8 active, ring-64 resolved), DilemmaType (I–V), DilemmaResolution (5 вариантов); crystallize_to_experience_commands() → UCL InjectToken+BondTokens для EXPERIENCE domain; lineage_hash FNV-1a; 13 тестов | ✅ |
| axiom-neural | 28 || SubsystemId extension | axiom-experience: SubsystemId += Morality(7), Abstractions(8), Dilemmas(9); subsystem_to_u8, subsystem_to_level, engine.rs string mapping | ✅ |
| axiom-neural | 28 || Shell-TD-02 | axiom-arbiter: Experience.shell_registry (sutra_id→[u8;8]); shell_cosine() → cosine similarity; pattern_similarity расширена: shell 15% модификатор (identitiчный→×1.0, ортогональный→×0.85, нет в registry→×0.925); 6 unit-тестов; inject_anchor_tokens → set_shell_registry() | ✅ |
| axiom-neural | 28 || SEN-TD-01 V2.0 | SensoriumState поглощает BroadcastSnapshot (Фаза A); Serialize + BroadcastHandle.sensorium_live + update_sensorium (Фаза B); axiom-web types+store+client (Фаза C); BroadcastSnapshot удалён из axiom-runtime/agent/broadcasting (Фаза F); axiom-observe/tray не затронуты (D/E no-op) | ✅ |
| axiom-neural | 28 || BRD-TD-06 | axiom-broadcasting: test_pong_timeout_disconnects_silent_client — raw TCP + ручной WS handshake, Ping игнорируется, сервер закрывает соединение в pong_timeout | ✅ |
